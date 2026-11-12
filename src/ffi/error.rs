use std::{
    cell::RefCell,
    ffi::{
        c_char,
        CString,
    },
    panic::{
        catch_unwind,
        AssertUnwindSafe,
    },
};

#[repr(C)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum OxideStatus {
    Ok = 0,
    InvalidArgument = 1,
    InvalidOperation = 2,
    ContentError = 3,
    GraphicsError = 4,
    InternalError = 5,
}

thread_local! {
    static LAST_ERROR:
        RefCell<Option<CString>> =
        const {
            RefCell::new(None)
        };
}

fn sanitize_message(
    message: String,
) -> CString {
    CString::new(message)
        .unwrap_or_else(
            |_| {
                CString::new(
                    "Oxide error message contained an invalid null character"
                )
                .expect(
                    "static error message must be valid"
                )
            }
        )
}

pub(crate) fn set_last_error(
    message: impl Into<String>,
) {
    let message =
        sanitize_message(
            message.into()
        );

    LAST_ERROR.with(
        |slot| {
            *slot.borrow_mut() =
                Some(message);
        }
    );
}

pub(crate) fn clear_last_error() {
    LAST_ERROR.with(
        |slot| {
            *slot.borrow_mut() =
                None;
        }
    );
}

/// Executes an FFI operation behind a panic barrier.
///
/// No Rust panic is permitted to unwind across the C ABI.
pub(crate) fn ffi_boundary<F>(
    operation: F,
) -> OxideStatus
where
    F: FnOnce() -> OxideStatus,
{
    clear_last_error();

    match catch_unwind(
        AssertUnwindSafe(
            operation
        )
    ) {
        Ok(status) => status,

        Err(payload) => {
            let message =
                if let Some(message) =
                    payload.downcast_ref::<&str>()
                {
                    (*message).to_string()
                } else if let Some(message) =
                    payload.downcast_ref::<String>()
                {
                    message.clone()
                } else {
                    String::from(
                        "unknown Rust panic"
                    )
                };

            set_last_error(
                format!(
                    "internal Oxide panic: {}",
                    message
                )
            );

            OxideStatus::InternalError
        }
    }
}

/// Returns the most recent error message for the calling thread.
///
/// The returned pointer is owned by Oxide. Foreign code must
/// not free or modify it. It remains valid until another Oxide
/// FFI call on the same thread changes the last error.
#[unsafe(no_mangle)]
pub extern "C" fn oxide_last_error_message()
    -> *const c_char
{
    LAST_ERROR.with(
        |slot| {
            slot.borrow()
                .as_ref()
                .map_or(
                    std::ptr::null(),
                    |message| {
                        message.as_ptr()
                    },
                )
        }
    )
}
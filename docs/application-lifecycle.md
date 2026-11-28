# Application lifecycle and errors

An application creates its event loop, window, and graphics before calling
`Initialize`/`initialize`. Each frame calls update and then draw. Run the game on
the process's main event-loop thread. The current window backend permits one
event loop per process; an application cannot be restarted in that process.
Nested and concurrent Oxide runs are rejected without changing the active game.

## Engine failures

Rust's `app::run` returns `Result<(), AppError>`. Expected event-loop, window,
graphics initialization, frame, presentation, and resize failures return errors
instead of panicking inside the platform callback. `AppError::stage()` identifies
the failed operation, its display message includes the cause, and `Error::source`
retains the error chain, including graphics/HRESULT information.

```rust,ignore
fn main() -> Result<(), oxide::app::AppError> {
    oxide::app::run(MyGame::new(), oxide::app::AppConfig::default())
}
```

C's `oxide_run` returns the existing status codes: invalid configuration uses
`OXIDE_STATUS_INVALID_ARGUMENT`, an overlapping run uses
`OXIDE_STATUS_INVALID_OPERATION`, and graphics startup/frame/resize failures use
`OXIDE_STATUS_GRAPHICS_ERROR`. Window and event-loop failures use
`OXIDE_STATUS_INTERNAL_ERROR`. Read `oxide_last_error_message()` before another
FFI call replaces that thread's diagnostic. C# and Python raise the corresponding
binding exception with this message.

Initial window dimensions must be positive; managed string titles must not
contain NUL. Invalid configuration is checked before creating the event loop.
Validating a C#/Python configuration does not consume that Game instance's one
run. Once a native run is attempted, use a new process to run another game.

The D3D12 debug layer is optional, including in development builds. Oxide enables
it once if it is available, and continues without it otherwise. Swap-chain
creation does not require the DXGI debug factory. End users need neither the
Graphics Tools optional Windows feature nor a separately configured debug DLL.
A compatible hardware D3D12 adapter is still required by the production renderer.

## Callback failures and exit

C# and Python catch callback exceptions before returning through the C ABI,
retain the first exception, and request exit immediately. Once native execution
returns, they rethrow that same exception with its original stack/traceback.
This also handles Python `KeyboardInterrupt` and `SystemExit`. Later native
errors do not replace the callback exception. A Graphics object expires even
when its draw callback throws.

The runtime checks exit/failure state after initialize, update, and draw:

- An exit in initialize prevents update and draw.
- An exit in update prevents beginning a graphics frame or calling draw.
- An exit in draw prevents frame submission/presentation and later callbacks.
- A graphics failure stops further frame work and returns the first engine error.

No native callback may unwind a foreign-language exception. C and C++ callers
must catch their own callback failures, retain them, call `oxide_app_exit()`,
return normally from the callback, then report/rethrow after `oxide_run` returns.
The existing C ABI remains the C++ integration interface.

Rust game callback panics are caught while inside the platform callback. After
the event loop returns and window/graphics teardown finishes, Oxide resumes the
original panic payload. Rust's panic hook still reports the original panic site;
user panics are not converted into successful exits or ordinary engine errors.

Per-run pending commands, exit state, window dimensions, focus, and fullscreen
state are cleared during teardown on success, engine failure, or resumed panic.
Window commands belong to the active run; issue setup commands in initialize.
Resize commands from draw are applied after the frame has been submitted, so
resources are never resized while their command list is being recorded.

## Regression checks

All checks live under `tests/`. The private Rust harness exercises callback stop
boundaries, preserved panic payloads, first-failure/cause retention, session cleanup,
and invalid startup configurations without requiring a game window. Existing WARP
readbacks continue to validate rendering after removing obsolete shadow helpers.
C# and Python checks exercise callback suppression, original exception identity
and stack/traceback, invalid configuration, and draw-handle invalidation. Python's
binding harness substitutes native loop calls; it does not require an interactive
window for failure-path checks.

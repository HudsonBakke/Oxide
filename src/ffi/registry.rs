use std::collections::HashMap;

pub(crate) struct Registry<T> {
    next_handle: u64,
    values: HashMap<u64, T>,
}

impl<T> Registry<T> {
    pub(crate) fn new() -> Self {
        Self {
            next_handle: 1,
            values: HashMap::new(),
        }
    }

    pub(crate) fn insert(&mut self, value: T) -> u64 {
        let handle = self.next_handle;

        self.next_handle = self.next_handle.wrapping_add(1);

        if self.next_handle == 0 {
            self.next_handle = 1;
        }

        self.values.insert(handle, value);

        handle
    }

    pub(crate) fn get(&self, handle: u64) -> Option<&T> {
        self.values.get(&handle)
    }

    pub(crate) fn get_mut(&mut self, handle: u64) -> Option<&mut T> {
        self.values.get_mut(&handle)
    }

    pub(crate) fn remove(&mut self, handle: u64) -> Option<T> {
        self.values.remove(&handle)
    }
}

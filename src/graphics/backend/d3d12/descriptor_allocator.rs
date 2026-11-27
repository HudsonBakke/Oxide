use std::{collections::BTreeMap, ops::Range};
use windows::{core::{Error, Result}, Win32::Foundation::{E_INVALIDARG, E_OUTOFMEMORY}};

/// Allocates contiguous ranges while preserving the renderer's reserved slots.
#[derive(Clone)]
pub(crate) struct DescriptorAllocator {
    start: u32,
    end: u32,
    free: Vec<Range<u32>>,
    allocated: BTreeMap<u32, u32>,
}

impl DescriptorAllocator {
    pub(crate) fn new(start: u32, end: u32) -> Self {
        Self { start, end, free: vec![start..end], allocated: BTreeMap::new() }
    }

    pub(crate) fn used(&self) -> u32 {
        self.allocated.values().sum()
    }

    pub(crate) fn allocate(&mut self, count: u32) -> Result<u32> {
        if count == 0 {
            return Err(Error::new(E_INVALIDARG, "descriptor allocation must be nonzero"));
        }
        let Some(index) = self.free.iter().position(|range| range.end - range.start >= count) else {
            let available: u32 = self.free.iter().map(|range| range.end - range.start).sum();
            let largest = self.free.iter().map(|range| range.end - range.start).max().unwrap_or(0);
            return Err(Error::new(E_OUTOFMEMORY, format!(
                "GPU descriptor capacity exhausted: requested {count} contiguous slots; {available} free, largest range {largest}, total ordinary capacity {}. Reduce simultaneously live textures/material combinations or finish the frame before submitting more assets.",
                self.end - self.start,
            )));
        };
        let start = self.free[index].start;
        self.free[index].start += count;
        if self.free[index].is_empty() { self.free.remove(index); }
        self.allocated.insert(start, count);
        Ok(start)
    }

    pub(crate) fn release(&mut self, start: u32, count: u32) -> bool {
        if self.allocated.get(&start) != Some(&count) { return false; }
        self.allocated.remove(&start);
        self.free.push(start..start + count);
        self.free.sort_by_key(|range| range.start);
        let mut merged: Vec<Range<u32>> = Vec::with_capacity(self.free.len());
        for range in self.free.drain(..) {
            if let Some(last) = merged.last_mut() {
                if last.end == range.start { last.end = range.end; continue; }
            }
            merged.push(range);
        }
        self.free = merged;
        true
    }
}

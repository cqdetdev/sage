use crate::program::Program;
use std::mem::size_of;
use winapi::{shared::ntdef::HANDLE, um::memoryapi::ReadProcessMemory};

pub struct Mem<'a> {
    pub p: &'a Program,
}

impl<'a> Mem<'a> {
    pub fn new(p: &'a Program) -> Self {
        Self { p }
    }

    pub unsafe fn addr_from_ptr(handle: HANDLE, game_id: usize, offsets: &[usize]) -> usize {
        let mut address: usize = game_id;
        for i in 0..offsets.len() {
            ReadProcessMemory(
                handle,
                address as *const _,
                &mut address as *mut _ as *mut _,
                size_of::<u64>(),
                0 as *mut _,
            );
            address = address + offsets[i];
        }
        address
    }
}

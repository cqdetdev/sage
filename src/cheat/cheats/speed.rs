use crate::cheat::RunnableCheat;
use crate::mem::Mem;
use crate::util::CVoidWrapper;
use std::mem::size_of_val;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};
use winapi::um::memoryapi::WriteProcessMemory;

pub struct Speed;

impl RunnableCheat for Speed {
    unsafe fn run(&self, handle: Arc<CVoidWrapper>, game_id: usize, val: usize) -> JoinHandle<()> {
        let offsets = self.offsets();
        spawn(move || {
            let speed_addr = Mem::addr_from_ptr(handle.0 as *mut _, game_id, offsets);
            let mut speed = 0x3f8ccccd;
            loop {
                WriteProcessMemory(
                    handle.0 as *mut _,
                    speed_addr as *mut _,
                    &mut speed as *mut _ as *mut _,
                    size_of_val(&speed),
                    0 as *mut _,
                );
            }
        })
    }

    fn name(&self) -> &'static str {
        "speed"
    }

    fn offsets(&self) -> &'static [usize] {
        &[0x0, 0x18, 0xB8, 0x4A0, 0x18, 0x1F0, 0x9C]
    }
}

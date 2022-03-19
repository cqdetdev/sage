use crate::util::CVoidWrapper;
use std::sync::Arc;
use std::thread::JoinHandle;

pub trait RunnableCheat {
    unsafe fn run(&self, handle: Arc<CVoidWrapper>, game_id: usize, val: usize) -> JoinHandle<()>;
    fn name(&self) -> &'static str;
    fn offsets(&self) -> &'static [usize];
}

use std::ffi::c_void;

pub struct CVoidWrapper(pub *mut c_void);

unsafe impl Send for CVoidWrapper {}
unsafe impl Sync for CVoidWrapper {}

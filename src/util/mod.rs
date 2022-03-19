pub mod c_void_wrapper;
pub use c_void_wrapper::CVoidWrapper;

pub struct Util;

impl Util {
    pub fn clean_str(s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
    }
}

use libc::c_char;
use std::ffi::CStr;

#[inline]
pub fn ptr_to_str(ptr: *const c_char) -> &'static str {
    unsafe {
        CStr::from_ptr(ptr)
            .to_str()
            .expect("Failed to convert raw pointer to strings")
    }
}

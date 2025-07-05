use std::ffi::{c_char, CString};

use sdl2::libc;

pub fn str_to_cstr<S>(str: S) -> *const c_char
where
    S: Into<String>,
{
    let mut s: String = str.into();

    let cs = CString::new(s).unwrap();

    cs.as_ptr()
}

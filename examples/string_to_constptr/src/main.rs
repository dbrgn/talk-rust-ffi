extern crate libc;

use std::ffi::CString;
use libc::c_char;

fn main() {
    let s: String = "Hello".to_string();
    let cs: CString = CString::new(s).unwrap();
    let ptr: *const c_char = cs.into_raw();
}

//! # fltk-rs-fluid
//! 
//! A Rust crate wrapping FLTK's fluid (RAD tool).
//! 
//! It allows installing fluid via cargo-install:
//! 
//! ```ignored
//! $ cargo install fltk-fluid
//! ```
//! 

use std::ffi::CString;
use std::os::raw::*;

extern "C" {
    pub fn fluid_main(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut args: Vec<*mut c_char> = args.iter().map(|s| CString::new(s.as_str()).unwrap().into_raw()).collect();
    unsafe {
        fluid_main(args.len() as i32, args.as_mut_ptr());
    }
}

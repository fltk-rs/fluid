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
use std::{env, os::raw};

extern "C" {
    pub fn fluid_main(argc: raw::c_int, argv: *mut *mut raw::c_char) -> raw::c_int;
}

fn main() {
    let mut args: Vec<_> = env::args()
        .into_iter()
        .map(|s| CString::new(s).unwrap().into_raw())
        .collect();
    unsafe {
        fluid_main(args.len() as i32, args.as_mut_ptr());
    }
}

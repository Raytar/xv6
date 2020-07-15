#![no_std]
#![no_main]
#![feature(start)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod bindings;
use bindings::printf;
// use core::panic::PanicInfo;
extern crate cstr_core;
use cstr_core::CStr;

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[panic_handler]
fn panic(__info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main(argc: isize, argv: *const *const u8) -> isize {
    let msg = CStr::from_bytes_with_nul(b"Hello world\n\0").unwrap();
    unsafe {
        printf(1, msg.as_ptr());
    }
    0
}

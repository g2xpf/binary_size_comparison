#![feature(start, lang_items)]
#![no_std]
#![no_main]

extern crate libc;

use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    pub fn printf(format: *const u8, ...) -> i32;
}

#[no_mangle]
pub extern "C" fn main(_nargs: i32, _args: *const *const u8) -> i32 {
    unsafe {
        printf(b"Hello, world!" as *const u8);
    }
    0
}

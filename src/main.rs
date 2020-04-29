#![feature(start, libc, lang_items)]
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate libc;

use alloc::boxed::Box;
use core::panic::PanicInfo;

mod calloc;
use calloc::ESystem;

#[global_allocator]
static GLOBAL: ESystem = ESystem;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}


// A list of C functions that are being imported
extern "C" {
    pub fn printf(format: *const u8, ...) -> i32;
}

#[no_mangle]
pub extern "C" fn main(_nargs: i32, _args: *const *const u8) -> i32 {
    let fmt: &str = "%s %d Hello world\n\0";
    unsafe {
        printf(fmt.as_ptr(), *_args, 0);
    }
    let mut a = Box::new([0i32; 100]);
    for idx in 0..a.len() {
        a[idx] = idx as i32;
    }
    for idx in 0..a.len() {
        unsafe {
            printf("%d \0".as_ptr(), a[idx]);
        }
    }
    0
}


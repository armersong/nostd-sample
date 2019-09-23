#![feature(start, libc, lang_items)]
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

// The libc crate allows importing functions from C.
extern crate libc;
extern crate alloc;

use core::panic::PanicInfo;
use alloc::boxed::Box;

mod calloc;
use calloc::ESystem;

#[global_allocator]
static GLOBAL:ESystem = ESystem;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// A list of C functions that are being imported
extern {
    pub fn printf(format: *const u8, ...) -> i32;
}

#[no_mangle]
// The main function, with its input arguments ignored, and an exit status is returned
pub extern fn main(_nargs: i32, _args: *const *const u8) -> i32 {
    // Print "Hello, World" to stdout using printf
    let fmt:&str = "%s %d Hello world\n\0";
    unsafe { 
        printf(fmt.as_ptr(),  *_args, 0);
    }
    // Exit with a return status of 0.
    let mut a = Box::new([0i32;100]);
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

#[lang = "eh_personality"] extern fn eh_personality() {}
//#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { panic!() }
//

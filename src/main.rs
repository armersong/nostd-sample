#![feature(start, libc, lang_items)]
#![no_std]
#![no_main]

// The libc crate allows importing functions from C.
extern crate libc;
use core::panic::PanicInfo;
use libc::c_uint;

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
    let mut count:u16 = 0;
    let fmt:&str = "%s %d Hello world\n\0";
    while count < 100 {
        unsafe { 
            printf(fmt.as_ptr(),  *_args, count as c_uint);
        }
        count += 1;
    }
    // Exit with a return status of 0.
    0
}

#[lang = "eh_personality"] extern fn eh_personality() {}
//#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { panic!() }
//

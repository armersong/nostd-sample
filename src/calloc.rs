#![allow(dead_code)]
/// copy from libstd

use core::alloc::{GlobalAlloc, Layout};
use core::cmp;
use core::ptr;

#[cfg(all(any(
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "asmjs",
    target_arch = "wasm32"
)))]
pub const MIN_ALIGN: usize = 8;

#[cfg(all(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "mips64",
    target_arch = "s390x",
    target_arch = "sparc64"
)))]
pub const MIN_ALIGN: usize = 16;

pub struct ESystem;

extern "C" {
    pub fn printf(format: *const u8, ...) -> i32;
}

unsafe impl GlobalAlloc for ESystem {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        printf("alloc %d\n\0".as_ptr(), layout.size());
        if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            libc::malloc(layout.size()) as *mut u8
        } else {
            #[cfg(target_os = "macos")]
            {
                if layout.align() > (1 << 31) {
                    return ptr::null_mut();
                }
            }
            aligned_malloc(&layout)
        }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        printf("alloc_zeroed %d\n\0".as_ptr(), layout.size());
        if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            libc::calloc(layout.size(), 1) as *mut u8
        } else {
            let ptr = self.alloc(layout.clone());
            if !ptr.is_null() {
                ptr::write_bytes(ptr, 0, layout.size());
            }
            ptr
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        printf("dealloc %p\n\0".as_ptr(), ptr);
        libc::free(ptr as *mut libc::c_void)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        printf("realloc %p, %d\n\0".as_ptr(), ptr, new_size);
        if layout.align() <= MIN_ALIGN && layout.align() <= new_size {
            libc::realloc(ptr as *mut libc::c_void, new_size) as *mut u8
        } else {
            realloc_fallback(self, ptr, layout, new_size)
        }
    }
}

pub unsafe fn realloc_fallback(
    alloc: &ESystem,
    ptr: *mut u8,
    old_layout: Layout,
    new_size: usize,
) -> *mut u8 {
    let new_layout = Layout::from_size_align_unchecked(new_size, old_layout.align());

    let new_ptr = GlobalAlloc::alloc(alloc, new_layout);
    if !new_ptr.is_null() {
        let size = cmp::min(old_layout.size(), new_size);
        ptr::copy_nonoverlapping(ptr, new_ptr, size);
        GlobalAlloc::dealloc(alloc, ptr, old_layout);
    }
    new_ptr
}

#[cfg(any(
    target_os = "android",
    target_os = "hermit",
    target_os = "redox",
    target_os = "solaris"
))]
#[inline]
unsafe fn aligned_malloc(layout: &Layout) -> *mut u8 {
    libc::memalign(layout.align(), layout.size()) as *mut u8
}

#[cfg(not(any(
    target_os = "android",
    target_os = "hermit",
    target_os = "redox",
    target_os = "solaris"
)))]
#[inline]
unsafe fn aligned_malloc(layout: &Layout) -> *mut u8 {
    let mut out = ptr::null_mut();
    let ret = libc::posix_memalign(&mut out, layout.align(), layout.size());
    if ret != 0 {
        ptr::null_mut()
    } else {
        out as *mut u8
    }
}

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    unsafe {
        printf("Error! alloc_error size %d\n\0".as_ptr(), layout.size());
    }
    loop {}
}

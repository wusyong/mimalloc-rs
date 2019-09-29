#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use lmimalloc_sys::*;

// Copied from https://github.com/rust-lang/rust/blob/master/src/libstd/sys_common/alloc.rs
#[cfg(all(any(
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "asmjs",
    target_arch = "wasm32"
)))]
const MIN_ALIGN: usize = 8;

#[cfg(all(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "mips64",
    target_arch = "s390x",
    target_arch = "sparc64"
)))]
const MIN_ALIGN: usize = 16;

pub struct MiMalloc;

impl MiMalloc {
    fn is_aligned(layout: &Layout) -> bool {
        layout.align() <= MIN_ALIGN && layout.align() <= layout.size()
    }
}

unsafe impl GlobalAlloc for MiMalloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if Self::is_aligned(&layout) {
            mi_malloc(layout.size()) as *mut u8
        } else {
            mi_malloc_aligned(layout.size(), layout.align()) as *mut u8
        }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if Self::is_aligned(&layout) {
            mi_zalloc(layout.size()) as *mut u8
        } else {
            mi_zalloc_aligned(layout.size(), layout.align()) as *mut u8
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mi_free(ptr as *mut _);
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if Self::is_aligned(&layout) {
            mi_realloc(ptr as *mut _, new_size) as *mut u8
        } else {
            mi_realloc_aligned(ptr as *mut _, new_size, layout.align()) as *mut u8
        }
    }
}

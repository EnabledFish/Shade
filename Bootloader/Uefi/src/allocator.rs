use alloc::alloc::{alloc, dealloc};
use core::alloc::{Layout, LayoutError};

pub fn layout(size: usize) -> Result<Layout, LayoutError> {
    Layout::array::<u8>(size)
}

pub fn malloc<T: Sized>(size: usize) -> *mut T {
    unsafe {
        alloc(layout(size).unwrap()) as *mut T
    }
}

pub fn free<T: Sized>(pointer: *mut T, size: usize) {
    unsafe {
        dealloc(pointer as *mut u8, layout(size).unwrap());
    }
}

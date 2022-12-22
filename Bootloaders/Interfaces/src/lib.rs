#![no_std]

pub mod video;
pub mod kernel;
pub mod boot;
pub mod optional;

pub use optional::*;

#[inline]
pub const fn u64_cast_to_pointer<T: Sized>(value: u64)->*mut T{
    value as usize as *mut T
}

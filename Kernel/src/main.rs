#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[export_name = "_start"]
pub unsafe extern fn kernel_entry() {}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

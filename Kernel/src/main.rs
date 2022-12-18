#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[export_name = "_start"]
pub extern "C" fn kernel_entry() -> usize {
    return message()
}

fn message() -> usize {
    23333
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

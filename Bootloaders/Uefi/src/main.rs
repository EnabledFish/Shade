#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(deprecated)]
#![allow(dead_code)]

extern crate alloc;

use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi_services::*;

static mut SYSTEM_TABLE: Option<SystemTable<Boot>> = None;
static mut ENTRY_HANDLE: Option<Handle> = None;

#[entry]
pub unsafe fn main(handle: Handle, system_table: SystemTable<Boot>) -> Status {
    // Initialize the entry handle and the system table before calling any interfaces.
    init_handle_and_system_table(handle, system_table);
    init(get_system_table()).unwrap();

    // Clear the screen before logging.
    get_system_table().stdout().clear().unwrap();
    println!("[ShadeBootloader] Initializing...");

    loop {}
}

unsafe fn init_handle_and_system_table(handle: Handle, system_table: SystemTable<Boot>) {
    ENTRY_HANDLE = Some(handle);
    SYSTEM_TABLE = Some(system_table);
}

fn get_system_table() -> &'static mut SystemTable<Boot> {
    unsafe {
        if let Some(system_table) = SYSTEM_TABLE.as_mut() {
            return system_table;
        } else {
            panic!("[ShadeBootloader] The system table was not initialized.")
        }
    }
}

fn get_entry_handle() -> Handle {
    unsafe {
        if let Some(entry_handle) = ENTRY_HANDLE {
            return entry_handle;
        } else {
            panic!("[ShadeBootloader] The entry handle was not initialized.")
        }
    }
}

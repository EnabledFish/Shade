#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(deprecated)]
#![allow(dead_code)]

mod kernel;
mod framebuffer;
mod filesystem;
mod string;
mod allocator;

extern crate alloc;

use alloc::vec::Vec;
use core::mem::transmute;
use core::ptr::slice_from_raw_parts;
use goblin::elf::Elf;
use uefi::prelude::*;
use uefi_services::{print, println};
use shade_bootloader_interface::bootloader::BootloaderInfo;
use crate::framebuffer::get_framebuffer_info;
use crate::kernel::get_kernel_code;

static mut SYSTEM_TABLE: Option<SystemTable<Boot>> = None;
static mut ENTRY_HANDLE: Option<Handle> = None;

#[entry]
fn main(handle: Handle, system_table: SystemTable<Boot>) -> Status {
    // Initialize the entry handle and the system table before calling any interfaces.
    unsafe {
        init_handle_and_system_table(handle, system_table);
        uefi_services::init(get_system_table()).unwrap();
    }

    // Clear the screen before logging.
    get_system_table().stdout().clear().unwrap();
    println!("[ShadeBootloader] Initializing...");

    // Locate the framebuffer.
    let framebuffer = get_framebuffer_info();
    println!("[ShadeBootloader] Framebuffer address located: {:?}.", framebuffer.get_buffer().get());
    println!("[ShadeBootloader] Framebuffer size: {} pixels.", framebuffer.get_size());
    println!("[ShadeBootloader] Screen resolution: {}x{}.", framebuffer.get_width(), framebuffer.get_height());

    // Construct the info of the bootloader.
    let _bootloader_info = BootloaderInfo::new(Some(framebuffer));

    // Get kernel code.
    let code = get_kernel_code();
    println!("[ShadeBootloader] Kernel code loaded at: {:?}", code.get_code());
    println!("[ShadeBootloader] Kernel code size: {}", code.get_size());

    unsafe {
        println!("[ShadeBootloader] Kernel code slice:");
        print_code_slice(code.get_code());
    }

    let code_slice = unsafe {
        slice_from_raw_parts(
            code.get_code(),
            code.get_size(),
        ).as_ref().unwrap()
    };

    // Parse the entry point of the kernel.
    let elf = Elf::parse(code_slice).unwrap();
    let entry = elf.entry as usize;
    let phdrs_executable = elf
        .program_headers.iter()
        .filter(|ph| ph.is_executable())
        .collect::<Vec<_>>();
    let entry_phdr = phdrs_executable.get(0).unwrap();
    let entry_offset = entry - entry_phdr.p_vaddr as usize + entry_phdr.p_offset as usize;
    let entry_addr = unsafe { code.get_code().add(entry_offset) };
    println!("[ShadeBootloader] Kernel entry offset: {}.", entry_offset);
    println!("[ShadeBootloader] Kernel entry function pointer: {:?}", entry_addr);

    unsafe {
        println!("[ShadeBootloader] Kernel entry code slice:");
        print_code_slice(entry_addr);
        let entry_func = transmute::<_, extern "C" fn() -> usize>(entry_addr);
        println!("[ShadeKernel] Kernel exited with: {}", entry_func());
    }

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

unsafe fn print_code_slice(code: *const u8) {
    let code_slice = slice_from_raw_parts(code, 128).as_ref().unwrap();
    print!("    ");
    for i in 0..code_slice.len() {
        if i != 0 && i % 16 == 0 {
            println!();
            print!("    ");
        }
        print!("{:02X}; ", code_slice.get(i).unwrap());
    }
    println!();
}

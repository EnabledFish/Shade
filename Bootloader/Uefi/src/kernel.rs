use alloc::boxed::Box;
use core::ptr::slice_from_raw_parts_mut;
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode};
use crate::allocator::{free, malloc};
use crate::filesystem::{file_handle_open, get_filesystem_root};

pub struct KernelCode {
    code: *const u8,
    size: usize,
}

impl KernelCode {
    pub fn get_code(&self) -> *const u8 {
        self.code
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

impl Drop for KernelCode {
    fn drop(&mut self) {
        free(self.code as *mut u8, self.size)
    }
}

pub fn get_kernel_code() -> KernelCode {
    let mut dir_root = get_filesystem_root();
    let mut dir_shade = file_handle_open(
        dir_root.handle(),
        "Shade",
        FileMode::Read,
        FileAttribute::DIRECTORY,
    ).unwrap();
    let mut dir_shade_kernel = file_handle_open(
        &mut dir_shade,
        "Kernel",
        FileMode::Read,
        FileAttribute::DIRECTORY,
    ).unwrap();
    let file_kernel = file_handle_open(
        &mut dir_shade_kernel,
        "Kernel.pgm",
        FileMode::Read,
        FileAttribute::READ_ONLY,
    ).unwrap();
    let mut buf = Box::new([0u8; 1024]);
    let mut reg_file = file_kernel.into_regular_file().unwrap();
    let info = reg_file.get_info::<FileInfo>(buf.as_mut()).unwrap();
    let size = info.file_size() as usize;
    let kernel_code = malloc::<u8>(size);
    unsafe {
        reg_file.read(
            slice_from_raw_parts_mut(kernel_code, size).as_mut().unwrap()
        ).unwrap();
    }
    KernelCode {
        code: kernel_code,
        size,
    }
}

use core::cell::UnsafeCell;
use core::str::FromStr;
use uefi::proto::media::file::{Directory, File, FileAttribute, FileHandle, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;
use crate::get_system_table;
use crate::string::CString16;

pub fn file_handle_open(handle: &mut FileHandle, name: &str, mode: FileMode, attr: FileAttribute) -> uefi::Result<FileHandle> {
    let name_16 = CString16::from_str(name).unwrap();
    handle.open(name_16.to_cstr16(), mode, attr)
}

pub fn get_filesystem_protocol() -> &'static UnsafeCell<SimpleFileSystem> {
    unsafe {
        get_system_table().boot_services().locate_protocol::<SimpleFileSystem>()
    }.unwrap()
}

pub fn get_filesystem_root() -> Directory {
    unsafe {
        get_filesystem_protocol().get().as_mut().unwrap().open_volume().unwrap()
    }
}

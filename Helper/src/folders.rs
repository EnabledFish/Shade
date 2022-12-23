use std::path::{Path, PathBuf};
use fs_err::{canonicalize, create_dir};
use fs_extra::dir::CopyOptions;

pub fn create_folder_if_not_exist(path: PathBuf) -> PathBuf {
    if !path.exists() {
        create_dir(&path).unwrap();
        println!("[ShadeHelper] Created one folder: \"{}\".", path.to_str().unwrap());
    }
    path
}

pub fn copy_folder<F: AsRef<Path>, T: AsRef<Path>>(from: F, to: T) {
    if !to.as_ref().exists() {
        create_dir(&to).unwrap();
    }
    fs_extra::dir::copy(
        &from,
        &to,
        &CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 64000,
            copy_inside: false,
            content_only: true,
            depth: 0,
        },
    ).unwrap();
    println!(
        "[ShadeHelper] Copied one folder: \"{}\" -> \"{}\".",
        from.as_ref().to_str().unwrap(),
        to.as_ref().to_str().unwrap()
    );
}

pub fn copy_folder_if_not_exist<F: AsRef<Path>>(from: F, to: PathBuf) -> PathBuf {
    if !to.exists() {
        copy_folder(from, &to);
    }
    to
}

pub fn copy_folder_if_not_exist_join<F: AsRef<Path>>(from: F, to: PathBuf, join: &str) -> PathBuf {
    if !to.join(join).exists() {
        copy_folder(from, &to);
    }
    to.join(join)
}

pub fn folder_root() -> PathBuf {
    canonicalize(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
    ).unwrap()
}

pub fn file_root(name: &str) -> PathBuf {
    folder_root().join(name)
}

pub fn folder_base() -> PathBuf {
    file_root("Base")
}

pub fn folder_bootloaders() -> PathBuf {
    file_root("Bootloaders")
}

pub fn folder_bootloaders_uefi() -> PathBuf {
    folder_bootloaders().join("Uefi")
}

pub fn folder_bootloaders_uefi_base() -> PathBuf {
    folder_bootloaders_uefi().join("Base")
}

pub fn folder_build() -> PathBuf {
    create_folder_if_not_exist(file_root("Build"))
}

pub fn file_build(name: &str) -> PathBuf {
    folder_build().join(name)
}

pub fn folder_build_base() -> PathBuf {
    copy_folder_if_not_exist(
        folder_base(),
        file_build("Base"),
    )
}

pub fn folder_build_base_shade() -> PathBuf {
    folder_build_base().join("Shade")
}

pub fn folder_build_base_shade_kernel() -> PathBuf {
    folder_build_base_shade().join("Kernel")
}

pub fn file_build_base_shade_kernel(name: &str) -> PathBuf {
    folder_build_base_shade_kernel().join(name)
}

pub fn folder_build_base_efi_for_uefi() -> PathBuf {
    copy_folder_if_not_exist_join(
        folder_bootloaders_uefi_base(),
        folder_build_base(),
        "EFI",
    )
}

pub fn folder_build_base_efi_boot() -> PathBuf {
    folder_build_base_efi_for_uefi().join("Boot")
}

pub fn file_build_base_efi_boot_for_uefi(name: &str) -> PathBuf {
    folder_build_base_efi_boot().join(name)
}

pub fn folder_build_objects() -> PathBuf {
    create_folder_if_not_exist(file_build("Objects"))
}

pub fn file_build_objects(name: &str) -> PathBuf {
    folder_build_objects().join(name)
}

pub fn folder_drivers() -> PathBuf {
    file_root("Drivers")
}

pub fn folder_helper() -> PathBuf {
    file_root("Helper")
}

pub fn folder_kernel() -> PathBuf {
    file_root("Kernel")
}

pub fn folder_libraries() -> PathBuf {
    file_root("Libraries")
}

pub fn folder_resources() -> PathBuf {
    file_root("Resources")
}

pub fn file_resources(name: &str) -> PathBuf {
    folder_resources().join(name)
}

pub fn folder_userlands() -> PathBuf {
    file_root("Userlands")
}


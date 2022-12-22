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

pub fn folder_base() -> PathBuf {
    folder_root().join("Base")
}

pub fn folder_bootloaders() -> PathBuf {
    folder_root().join("Bootloaders")
}

pub fn folder_bootloaders_uefi() -> PathBuf {
    folder_bootloaders().join("Uefi")
}

pub fn folder_bootloaders_uefi_base() -> PathBuf {
    folder_bootloaders_uefi().join("Base")
}

pub fn folder_build() -> PathBuf {
    create_folder_if_not_exist(folder_root().join("Build"))
}

pub fn folder_build_base() -> PathBuf {
    copy_folder_if_not_exist(
        folder_base(),
        folder_build().join("Base"),
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
    create_folder_if_not_exist(folder_build().join("Objects"))
}

pub fn file_build_objects(name: &str) -> PathBuf {
    folder_build_objects().join(name)
}

pub fn folder_drivers() -> PathBuf {
    folder_root().join("Drivers")
}

pub fn folder_helper() -> PathBuf {
    folder_root().join("Helper")
}

pub fn folder_kernel() -> PathBuf {
    folder_root().join("Kernel")
}

pub fn folder_libraries() -> PathBuf {
    folder_root().join("Libraries")
}

pub fn folder_userlands() -> PathBuf {
    folder_root().join("Userlands")
}


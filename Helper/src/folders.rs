use std::path::PathBuf;
use fs_extra::dir::CopyOptions;

pub fn folder_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
}

pub fn folder_base() -> PathBuf {
    folder_root().join("Base")
}

pub fn folder_bootloaders() -> PathBuf {
    folder_root().join("Bootloaders")
}

pub fn folder_build() -> PathBuf {
    let path = folder_root().join("Build");
    if !path.exists() {
        fs_err::create_dir(&path).unwrap()
    }
    path
}

pub fn folder_build_base() -> PathBuf {
    let path = folder_build().join("Base");
    if !path.exists() {
        fs_err::create_dir(&path).unwrap();
        fs_extra::dir::copy(
            folder_root()
                .join("Base"),
            &path,
            &CopyOptions {
                overwrite: true,
                skip_exist: false,
                buffer_size: 64000,
                copy_inside: false,
                content_only: true,
                depth: 0,
            },
        ).unwrap();
    }
    path
}

pub fn folder_build_objects() -> PathBuf {
    let path = folder_build().join("Objects");
    if !path.exists() {
        fs_err::create_dir(&path).unwrap()
    }
    path
}

pub fn object_file(name: &str) -> PathBuf {
    folder_build_objects().join(name)
}

pub fn folder_drivers() -> PathBuf {
    folder_root().join("Drivers")
}

pub fn folder_helper() -> PathBuf {
    folder_root().join("Bootloaders")
}

pub fn folder_kernel() -> PathBuf {
    folder_root().join("Bootloaders")
}

pub fn folder_libraries() -> PathBuf {
    folder_root().join("Bootloaders")
}

pub fn folder_userlands() -> PathBuf {
    folder_root().join("Bootloaders")
}


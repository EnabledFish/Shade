use crate::process_main_from_iter;

#[test]
fn build_bootloader_uefi_x86_64() {
    process_main_from_iter(["build", "bootloader", "uefi", "-t", "x86_64"]);
}

#[test]
fn build_image() {
    process_main_from_iter(["build", "image"])
}

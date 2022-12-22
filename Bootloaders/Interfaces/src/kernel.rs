#[repr(C)]
#[derive(Copy, Clone)]
pub struct KernelInfo {
    pub pointer: u64,
    pub size: u64,
}

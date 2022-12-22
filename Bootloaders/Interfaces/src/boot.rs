use crate::kernel::KernelInfo;
use crate::video::VideoInfo;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BootInfo {
    pub kernel: KernelInfo,
    pub video: VideoInfo,
}

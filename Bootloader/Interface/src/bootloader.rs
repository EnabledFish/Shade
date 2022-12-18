use crate::framebuffer::FramebufferInfo;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BootloaderInfo {
    framebuffer_info: Option<FramebufferInfo>,
}

impl BootloaderInfo {
    pub fn new(
        framebuffer_info: Option<FramebufferInfo>,
    ) -> Self {
        Self {
            framebuffer_info,
        }
    }

    pub fn get_framebuffer_info(&self) -> Option<FramebufferInfo> {
        self.framebuffer_info
    }
}

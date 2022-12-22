use crate::video::framebuffer::FramebufferInfo;

pub mod framebuffer;

#[repr(u64)]
#[derive(Copy, Clone)]
pub enum VideoInfo{
    Framebuffer(FramebufferInfo),
}

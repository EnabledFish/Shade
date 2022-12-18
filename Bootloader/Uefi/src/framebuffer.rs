use uefi::proto::console::gop::GraphicsOutput;
use shade_bootloader_interface::framebuffer::{Framebuffer, FramebufferInfo};
use crate::get_system_table;

pub fn get_framebuffer_info() -> FramebufferInfo {
    unsafe {
        let gop = get_system_table().boot_services().locate_protocol::<GraphicsOutput>().unwrap();
        let gop = gop.get().as_mut().unwrap();
        let (width, height) = gop.current_mode_info().resolution();
        let mut framebuffer = gop.frame_buffer();
        let size = framebuffer.size();
        let pointer = framebuffer.as_mut_ptr();

        let framebuffer = Framebuffer::from_pointer(pointer);
        FramebufferInfo::new(
            framebuffer,
            width,
            height,
            size,
        )
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FramebufferInfo {
    pub pointer: u64,
    pub size: u64,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
}

#[repr(u64)]
#[derive(Copy, Clone)]
pub enum FramebufferPixelFormat {
    /// # Rgb
    /// Each pixel is 32-bit long, with 24-bit RGB,
    /// and the last byte is reserved.
    Rgb = 0,
    /// # Bgr
    /// Each pixel is 32-bit long, with 24-bit BGR,
    /// and the last byte is reserved.
    Bgr,
    /// # Bitmask
    /// Custom pixel format, check the associated
    /// bitmask.
    Bitmask,
    /// # BltOnly (Unsupported)
    /// The graphics mode does not support drawing
    /// directly to the frame buffer.
    ///
    /// This means you will have to use the `blt`
    /// function which will convert the graphics
    /// data to the device's internal pixel format.
    BltOnly,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PixelBitmask {
    /// # red
    /// The bits indicating the red channel.
    pub red: u32,
    /// # green
    /// The bits indicating the green channel.
    pub green: u32,
    /// # blue
    /// The bits indicating the blue channel.
    pub blue: u32,
    /// # reserved
    /// The reserved bits, which are ignored by the video hardware.
    pub reserved: u32,
}

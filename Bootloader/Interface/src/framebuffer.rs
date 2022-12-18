#[repr(C)]
#[derive(Copy, Clone)]
pub struct FramebufferInfo {
    buffer: Framebuffer,
    width: usize,
    height: usize,
    size: usize,
}

impl FramebufferInfo {
    pub fn new(
        buffer: Framebuffer,
        width: usize,
        height: usize,
        size: usize,
    ) -> Self {
        Self {
            buffer,
            width,
            height,
            size,
        }
    }

    #[inline(always)]
    pub fn get_buffer(&self) -> Framebuffer {
        self.buffer
    }

    #[inline(always)]
    pub fn get_width(&self) -> usize {
        self.width
    }

    #[inline(always)]
    pub fn get_height(&self) -> usize {
        self.width
    }

    #[inline(always)]
    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn write(&self, offset: usize, pixel: Pixel) -> Option<()> {
        self.buffer.write_unchecked(offset, self.size, pixel)
    }

    #[deprecated(
        note = "Reading the graphics memory is too slow."
    )]
    pub fn read(&self, offset: usize) -> Option<Pixel> {
        self.buffer.read_unchecked(offset, self.size)
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Framebuffer {
    pixels: *mut Pixel,
}

impl Framebuffer {
    pub const fn from_pointer<T>(pointer: *mut T) -> Self {
        Self {
            pixels: pointer as *mut Pixel
        }
    }

    #[inline(always)]
    pub const fn get(&self) -> *mut Pixel {
        self.pixels
    }

    #[inline(always)]
    pub const fn add(&self, count: usize) -> *mut Pixel {
        unsafe {
            self.pixels.add(count)
        }
    }

    #[inline(always)]
    pub unsafe fn write_checked(&self, offset: usize, pixel: Pixel) {
        self.add(offset).write_volatile(pixel);
    }

    #[inline(always)]
    pub unsafe fn read_checked(&self, offset: usize) -> Pixel {
        self.add(offset).read_volatile()
    }

    #[inline(always)]
    pub fn write_unchecked(&self, offset: usize, size: usize, pixel: Pixel) -> Option<()> {
        if offset < size {
            unsafe {
                self.add(offset).write_volatile(pixel);
            }
            Some(())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn read_unchecked(&self, offset: usize, size: usize) -> Option<Pixel> {
        if offset < size {
            Some(
                unsafe {
                    self.add(offset).read_volatile()
                }
            )
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Pixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}

impl Pixel {
    pub const fn new(
        red: u8,
        green: u8,
        blue: u8,
    ) -> Self {
        Self {
            blue,
            green,
            red,
            reserved: 0,
        }
    }

    pub const BLACK: Self = Pixel::new(0, 0, 0);
    pub const WHITE: Self = Pixel::new(255, 255, 255);
}

use image::RgbImage;

/// A 16bit pixel that has 5 red bits, 6 green bits and  5 blue bits
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Rgb565Pixel(pub u16);

impl Rgb565Pixel {
    const R_MASK: u16 = 0b1111_1000_0000_0000;
    const G_MASK: u16 = 0b0000_0111_1110_0000;
    const B_MASK: u16 = 0b0000_0000_0001_1111;

    /// Return the red component as a u8.
    ///
    /// The bits are shifted so that the result is between 0 and 255
    fn red(self) -> u8 {
        ((self.0 & Self::R_MASK) >> 8) as u8
    }
    /// Return the green component as a u8.
    ///
    /// The bits are shifted so that the result is between 0 and 255
    fn green(self) -> u8 {
        ((self.0 & Self::G_MASK) >> 3) as u8
    }
    /// Return the blue component as a u8.
    ///
    /// The bits are shifted so that the result is between 0 and 255
    fn blue(self) -> u8 {
        ((self.0 & Self::B_MASK) << 3) as u8
    }
}

impl Rgb565Pixel{
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(((r as u16 & 0b11111000) << 8) | ((g as u16 & 0b11111100) << 3) | (b as u16 >> 3))
    }
}

pub fn rgb888_to_rgb565_u16(img: &[u8], width: usize, height: usize) -> Vec<u16>{
    let mut rgb565 = vec![0u16; width * height];
    for (i, p) in img.chunks(3).enumerate(){
        let rgb565_pixel: Rgb565Pixel = Rgb565Pixel::from_rgb(p[0], p[1], p[2]);
        rgb565[i] = rgb565_pixel.0;
    }
    rgb565
}

pub fn rgb565_u16_image_to_rgb888(rgb565: &[u16], width: u32, height: u32) -> RgbImage{
    let mut rgb = RgbImage::new(width, height);
    for (i, p) in rgb.pixels_mut().enumerate(){
        let rgb565_pixel = Rgb565Pixel(rgb565[i]);
        p[0] = rgb565_pixel.red();
        p[1] = rgb565_pixel.green();
        p[2] = rgb565_pixel.blue();
    }
    rgb
}

pub fn slice_u16_to_slice_u8(arr: &[u16]) -> &[u8] {
    unsafe {
        let len = arr.len() * std::mem::size_of::<u16>();
        let ptr = arr.as_ptr() as *const u8;
        std::slice::from_raw_parts(ptr, len)
    }
}

pub fn slice_u8_to_slice_u16(arr: &[u8]) -> &[u16] {
    unsafe {
        let len = arr.len() / std::mem::size_of::<u16>();
        let ptr = arr.as_ptr() as *const u16;
        std::slice::from_raw_parts(ptr, len)
    }
}

use std::iter::FromIterator;

pub fn float_to_u8(f: f32) -> u8 {
    (f * 255.0) as u8
}

pub type IntColor = (u8, u8, u8);

pub type FloatColor = (f32, f32, f32);

pub type GrayscaleColor = f32;

pub struct Image<P: Copy> {
    width: usize,
    height: usize,
    pixels: Vec<P>
}

impl<P: Copy> Image<P> {

    pub fn new(width: usize, height: usize) -> Self {
        Image { 
            width, 
            height, 
            pixels: Vec::with_capacity(width * height) 
        }
    }

    pub fn from_color(width: usize, height: usize, color: P) -> Self {
        Image {
            width,
            height,
            pixels: Vec::from_iter((0..(width*height)).map(|_| color))
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: i64, y: i64) -> &P {
        let index = self.pixel_index(x, y);
        &self.pixels[index]
    }

    pub fn set(&mut self, x: i64, y: i64, c: P) {
        let index = self.pixel_index(x, y);
        self.pixels[index] = c;
    }

    fn pixel_index(&self, x: i64, y: i64) -> usize {
        wrap_around(x, self.width) + 
        wrap_around(y, self.height) * self.width
    }
}

fn wrap_around(num: i64, space: usize) -> usize {
    let mut res = num;
    let space_i64 = space as i64;

    while res < 0 {
        res += space_i64;
    }
    res = res % space_i64;

    return res as usize;
}
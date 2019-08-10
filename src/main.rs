extern crate image;

use image::{ImageBuffer, Rgb};

use image_gen::generators::diamond_square::{generate_diamond_square};
use image_gen::image::{Image,float_to_u8};

const RESOLUTION: usize = 129;

fn main() {
    
    println!("Generating...");

    let image = generate_diamond_square(RESOLUTION, 0.5, 0.5);

    println!("Writing to png...");

    let image_buffer = image_to_buffer(&image);
    image_buffer.save("output.png").unwrap();

    println!("done");
}

fn image_to_buffer(image: &Image<f32>) -> ImageBuffer<Rgb<u8>,Vec<u8>> {
    let mut image_buffer: ImageBuffer<Rgb<u8>,Vec<u8>> = ImageBuffer::new(RESOLUTION as u32, RESOLUTION as u32);

    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            let color = *image.get(x as i64, y as i64);
            image_buffer.get_pixel_mut(x as u32, y as u32).data = [ float_to_u8(color), float_to_u8(color), float_to_u8(color) ];
        }
    }

    return image_buffer;
}
extern crate image;

use image::{ImageBuffer, Rgb};
use image_gen::image::{Image,float_to_u8};
use image_gen::generators::bricks::{generate_bricks};
use image_gen::generators::perlin_noise::{generate_perlin_noise};

const RESOLUTION: usize = 1024;

fn main() {
    
    println!("Generating...");
    let start = std::time::SystemTime::now();

    //let image = generate_perlin_noise(RESOLUTION, 13);
    let image = generate_bricks(RESOLUTION, 2, 8, vec![0.1, 0.4], vec![0.1, 0.1]);

    let end = std::time::SystemTime::now();
    let total = end.duration_since(start).unwrap();
    println!("Took {}s", total.as_millis() as f32 / 1000.0);

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
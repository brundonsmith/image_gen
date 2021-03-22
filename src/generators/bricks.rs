
use crate::image::{GrayscaleColor, Image};
use crate::utils::vec2::Vec2;

pub fn generate_bricks(size: usize, bricks_x: usize, bricks_y: usize, bevel: Vec2, gap: Vec2) -> Image<GrayscaleColor> {
    let mut image = Image::from_color(size, size, 0.0);
    apply_bricks(&mut image, bricks_x, bricks_y, bevel, gap);
    return image;
}

pub fn apply_bricks(image: &mut Image<GrayscaleColor>, bricks_x: usize, bricks_y: usize, bevel: Vec2, gap: Vec2) {
    let brick_size_x = image.width() / bricks_x;
    let brick_size_y = image.height() / bricks_y;

    // create first brick
    for x in 0..brick_size_x {
        for y in 0..brick_size_y {
            let x_bevel = get_bevel_height(x, brick_size_x, bevel.x);
            let y_bevel = get_bevel_height(y, brick_size_y, bevel.y);

            image.set(x as i64, y as i64, x_bevel.min(y_bevel));
        }
    }

    // copy and offset second row
    for x in 0..brick_size_x/2 {
        for y in brick_size_y..brick_size_y*2 {
            image.set(x as i64, y as i64, *image.get((x + brick_size_x/2) as i64, (y - brick_size_y) as i64));
        }
    }
    for x in brick_size_x/2..brick_size_x {
        for y in brick_size_y..brick_size_y*2 {
            image.set(x as i64, y as i64, *image.get((x - brick_size_x/2) as i64, (y - brick_size_y) as i64));
        }
    }

    // copy the rest
    for brick_x in 0..bricks_x {
        for brick_y in (0..bricks_y).step_by(2) {

            // skip the ones we already did
            if brick_x != 0 || (brick_y != 0 && brick_y != 1) {
                let start_x = brick_x * brick_size_x;
                let start_y = brick_y * brick_size_y;

                // copy chunk
                for x in 0..brick_size_x {
                    for y in 0..brick_size_y*2 {
                        image.set((start_x + x) as i64, (start_y + y) as i64, *image.get(x as i64, y as i64));
                    }
                }
            }
        }
    }
}

fn get_bevel_height(pixel: usize, brick_size: usize, bevel: f32) -> f32 {
    let distance_from_center = (pixel as f32 - (brick_size as f32 / 2.0)).abs();
    let portion_from_center = distance_from_center / (brick_size as f32 / 2.0);
    let portion_from_edge = 1.0 - portion_from_center;
    return (portion_from_edge / bevel).max(0.0).min(1.0);
}
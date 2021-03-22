
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

extern crate rand;
use rand::Rng;

extern crate crossbeam;

use crate::image::{GrayscaleColor, Image};

use super::utils::vec2::Vec2;

/**
 * Creates a new Image and runs apply_perlin_noise on it. 
 */
pub fn generate_perlin_noise(size: usize, grid_size: usize, midpoint: f32, scale: f32) -> Image<GrayscaleColor> {
    let mut image = Image::from_color(size, size, midpoint);
    add_perlin_noise(&mut image, grid_size, scale);
    return image;
}

pub fn add_perlin_noise(image: &mut Image<GrayscaleColor>, grid_size: usize, scale: f32) {
    crossbeam::scope(move |scope| {
        let cell_size_x = image.width() / (grid_size - 1);
        let cell_size_y = image.height() / (grid_size - 1);

        // initialize grid
        let mut grid: Vec<Vec<Vec2>> = Vec::with_capacity(grid_size);
        for i in 0..grid_size {
            grid.push(Vec::with_capacity(grid_size));

            for _ in 0..grid_size {
                grid[i].push(generate_random_vector(2));
            }
        }

        let arc_grid = Arc::new(grid);
        let mutex_image: Arc<Mutex<&mut Image<GrayscaleColor>>> = Arc::new(Mutex::new(image));

        for cell_x in 0..(grid_size - 1) {
            for cell_y in 0..(grid_size - 1) {
                let arc_grid_clone = Arc::clone(&arc_grid);
                let mutex_image_clone = Arc::clone(&mutex_image);

                scope.spawn(move |_| {
                    perlin_cell(mutex_image_clone, &arc_grid_clone, scale, cell_size_x, cell_size_y, cell_x, cell_y);
                });
            }
        }
    }).unwrap();
}

fn perlin_cell(mutex_image: Arc<Mutex<&mut Image<GrayscaleColor>>>, grid: &Vec<Vec<Vec2>>, scale: f32, cell_size_x: usize, cell_size_y: usize, cell_x: usize, cell_y: usize) {
    let mut buf = Vec::with_capacity(cell_size_x * cell_size_y);

    for x in 0..cell_size_x {
        for y in 0..cell_size_y {
            let pixel_vec = Vec2 {
                x: (x + (cell_x * cell_size_x)) as f32, 
                y: (y + (cell_y * cell_size_y)) as f32
            };

            let corner_nodes = [
                (cell_x, cell_y),
                (cell_x + 1, cell_y),
                (cell_x, cell_y + 1),
                (cell_x + 1, cell_y + 1),
            ];

            let dots: Vec<f32> = corner_nodes.iter().map(|corner| {

                // get distance of pixel from corner
                let corner_vec = Vec2 {
                    x: (corner.0 * cell_size_x) as f32, 
                    y: (corner.1 * cell_size_y) as f32
                };
                let mut distance = &pixel_vec - &corner_vec;
                distance.x /= cell_size_x as f32;
                distance.y /= cell_size_y as f32;

                // get random vec associated with corner
                let corner_gradient_vec = &grid[corner.0][corner.1];

                // compute the dot product and scale it to fit in 0..1
                return distance.dot(&corner_gradient_vec) * scale;
            }).collect();

            let interpolation_1 = serp(dots[0], dots[1], x as f32 / cell_size_x as f32);
            let interpolation_2 = serp(dots[2], dots[3], x as f32 / cell_size_x as f32);
            let final_interpolation = serp(interpolation_1, interpolation_2, y as f32 / cell_size_y as f32);

            //let image = mutex_image.lock().unwrap();
            buf.push(final_interpolation);
        }
    }

    let image: &mut Image<GrayscaleColor> = &mut mutex_image.lock().unwrap();
    for i in 0..buf.len() {
        let x = cell_x * cell_size_x + i / cell_size_y;
        let y = cell_y * cell_size_y + i % cell_size_y;

        image.add(x as i64, y as i64, buf[i]);
    }
}

fn generate_random_vector(_dimensions: usize) -> Vec2 {
    let mut vec = Vec2 {
        x: rand::thread_rng().gen_range(-1.0, 1.0),
        y: rand::thread_rng().gen_range(-1.0, 1.0),
    };

    while vec.len() > 1.0 {
        vec.x = rand::thread_rng().gen_range(-1.0, 1.0);
        vec.y = rand::thread_rng().gen_range(-1.0, 1.0);
    }

    vec.normalize();
    return vec;
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (b - a) * t + a
}

fn serp(a: f32, b: f32, t: f32) -> f32 {
    lerp(a, b, ((t - 0.5) * PI).sin() / 2.0 + 0.5)
}

fn smoothstep(a: f32, b: f32, t: f32) -> f32 {

    // Scale, bias and saturate x to 0..1 range
    let range = b - a;
    let portion = (t - a) / range;
    let portion = clamp(portion, 0.0, 1.0);

    // Evaluate polynomial
    return portion * portion * (3.0 - 2.0 * portion);
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    x.max(min).min(max)
}

/*
enum Grid {
    Dimension(Vec<Grid>),
    Cell(Vector)
}

impl Grid {
    fn new(size: usize, dimensions: usize) -> Self {
        let mut grid: Self = Grid::Dimension(Vec::with_capacity(size));

        for dim in 2..dimensions {
            grid = Grid::Dimension(Vec::with_capacity(size));
        }

        return grid;
    }

    fn all(&mut self, cb: FnMut) {

    }
}
*/

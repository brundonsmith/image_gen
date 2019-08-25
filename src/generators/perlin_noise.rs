
extern crate rand;
use rand::Rng;
use std::f32::consts::PI;

use crate::image::{GrayscaleColor, Image};
use crate::generators::utils::vec::{Vector,VectorMath};
/**
 * Creates a new Image and runs apply_perlin_noise on it. 
 */
pub fn generate_perlin_noise(size: usize, grid_size: usize) -> Image<GrayscaleColor> {
    let mut image = Image::from_color(size, size, 0.5);
    apply_perlin_noise(&mut image, grid_size);
    return image;
}

pub fn apply_perlin_noise(image: &mut Image<GrayscaleColor>, grid_size: usize) {
    let start = std::time::SystemTime::now();
    let mut focus_duration = std::time::Duration::new(0, 0);

    let cell_size_x = image.width() / (grid_size - 1);
    let cell_size_y = image.height() / (grid_size - 1);

    // initialize grid
    let mut grid: Vec<Vec<Vector>> = Vec::with_capacity(grid_size);
    for i in 0..grid_size {
        grid.push(Vec::with_capacity(grid_size));

        for _ in 0..grid_size {
            grid[i].push(generate_random_vector(2));
        }
    }

    for x in 0..image.width() {
        for y in 0..image.height() {
            let x_cell = x / cell_size_x;
            let y_cell = y / cell_size_y;
            let pixel_vec = vec![x as f32, y as f32];

            let corner_nodes = [
                (x_cell, y_cell),
                (x_cell + 1, y_cell),
                (x_cell, y_cell + 1),
                (x_cell + 1, y_cell + 1),
            ];

            let start_focus = std::time::SystemTime::now();
            let dots: Vec<f32> = corner_nodes.iter().map(|corner| {

                // get distance of pixel from corner
                let corner_vec = vec![
                    (corner.0 * cell_size_x) as f32, 
                    (corner.1 * cell_size_y) as f32
                ];
                let mut distance = pixel_vec.sub(&corner_vec);
                *distance.x() /= cell_size_x as f32;
                *distance.y() /= cell_size_y as f32;

                // get random vec associated with corner
                let corner_gradient_vec = &grid[corner.0][corner.1];

                // compute the dot product and scale it to fit in 0..1
                return distance.mul(corner_gradient_vec) / 2.0 + 0.5;
            }).collect();
            let end_focus = std::time::SystemTime::now();
            focus_duration += end_focus.duration_since(start_focus).unwrap();


            let relative_x = (x - corner_nodes[0].0 * cell_size_x) as f32;
            let relative_y = (y - corner_nodes[0].1 * cell_size_y) as f32;

            let interpolation_1 = serp(dots[0], dots[1], relative_x / cell_size_x as f32);
            let interpolation_2 = serp(dots[2], dots[3], relative_x / cell_size_x as f32);
            let final_interpolation = serp(interpolation_1, interpolation_2, relative_y / cell_size_y as f32);

            image.set(x as i64, y as i64, final_interpolation);
        }
    }

    let end = std::time::SystemTime::now();
    let total_duration = end.duration_since(start).unwrap();

    let fraction = focus_duration.as_millis() as f32 / total_duration.as_millis() as f32;

    println!("{}ms, {} of total time", focus_duration.as_millis(), fraction);
}

fn generate_random_vector(_dimensions: usize) -> Vector {
    let mut vec = vec![
        rand::thread_rng().gen_range(-1.0, 1.0),
        rand::thread_rng().gen_range(-1.0, 1.0),
    ];

    while vec.magnitude() > 1.0 {
        vec[0] = rand::thread_rng().gen_range(-1.0, 1.0);
        vec[1] = rand::thread_rng().gen_range(-1.0, 1.0);
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

use crate::image::{GrayscaleColor, Image};

pub enum HillShape {
    Constant,
    Linear,
    Sphere,
    Smooth,
}

pub fn generate_hill(size: usize, hill_location: (f32, f32), hill_shape: HillShape, hill_radius: f32, hill_height: f32) -> Image<GrayscaleColor> {
    let mut image = Image::from_color(size, size, 0.0);
    add_hill(&mut image, hill_location, hill_shape, hill_radius, hill_height);
    return image;
}

pub fn add_hill(image: &mut Image<GrayscaleColor>, hill_location: (f32, f32), hill_shape: HillShape, hill_radius: f32, hill_height: f32) {
    let size = image.width();

    assert!(hill_location.0 >= 0.0 && hill_location.0 < 1.0 && hill_location.1 >= 0.0 && hill_location.1 < 1.0, "Location x and y must both be >= 0 and < 1");
    let hill_location = (hill_location.0 * size as f32, hill_location.1 * size as f32);
    let hill_radius = hill_radius * size as f32;

    let radius_squared = hill_radius * hill_radius;

    match hill_shape {
        HillShape::Constant => {
            for x in 0..size {
                for y in 0..size {
                    let distance_squared = calculate_distance_squared(x, y, hill_location);

                    if distance_squared < radius_squared {
                        image.add(x as i64, y as i64, hill_height);
                    }
                }
            }
        },
        HillShape::Linear => {
            let radius = radius_squared.sqrt();

            for x in 0..size {
                for y in 0..size {
                    let distance_squared = calculate_distance_squared(x, y, hill_location);

                    if distance_squared < radius_squared {
                        let distance = distance_squared.sqrt();

                        image.add(x as i64, y as i64, hill_height - (distance / radius));
                    }
                }
            }
        },
        HillShape::Sphere => {
            for x in 0..size {
                for y in 0..size {
                    let distance_squared = calculate_distance_squared(x, y, hill_location);

                    if distance_squared < radius_squared {
                        image.add(x as i64, y as i64, hill_height - (distance_squared / radius_squared));
                    }
                }
            }
        },
        HillShape::Smooth => {
            let radius = radius_squared.sqrt();

            for x in 0..size {
                for y in 0..size {
                    let distance_squared = calculate_distance_squared(x, y, hill_location);

                    if distance_squared < radius_squared {
                        let distance = distance_squared.sqrt();
                        let s_x = 1.0 - (distance / radius);
                        let x_squared = s_x * s_x;
                        let s = 3.0 * x_squared - 2.0 * x_squared * s_x;

                        image.add(x as i64, y as i64, s);
                    }
                }
            }
        },
    }
}

fn calculate_distance_squared(x: usize, y: usize, hill_location: (f32, f32)) -> f32 {
    let x = x as f32 + 0.5;
    let y = y as f32 + 0.5;

    let dx = x - hill_location.0;
    let dy = y - hill_location.1;

    return dx * dx + dy * dy;
}
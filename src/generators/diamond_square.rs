
use std::collections::{HashSet};

extern crate rand;
use rand::Rng;

use crate::image::{GrayscaleColor, Image};

pub fn generate_diamond_square(size: usize, variance: f32, coarseness: f32) -> Image<GrayscaleColor> {
    let mut image = Image::from_color(size, size, 0.5);
    apply_diamond_square(&mut image, variance, coarseness);
    return image;
}

pub fn apply_diamond_square(image: &mut Image<GrayscaleColor>, variance: f32, coarseness: f32) {
    assert!(image.width() == image.height(), "Image width and height must be the same");
    assert!((image.width() - 1).is_power_of_two(), "Image width/height must be a power of two plus one");
    assert!(variance >= 0.0, "Variance should be greater than or equal to 0");
    assert!(variance <= 1.0, "Variance should be less than or equal to 1");
    assert!(coarseness >= 0.0, "Coarseness should be greater than or equal to 0");
    assert!(coarseness <= 1.0, "Coarseness should be less than or equal to 1");

    let size = image.width() as i64;
    let mut partitions = 1;
    while partitions < size - 1 {
        pass(image, partitions, variance, coarseness);
        partitions *= 2;
    }
}

fn pass(image: &mut Image<GrayscaleColor>, partitions: i64, variance: f32, coarseness: f32) {
    let image_size = image.width() as i64;
    let partition_size = (image_size - 1) / partitions;
    println!("partition size: {}", partition_size);

    let real_variance;
    {
        let pass_num = f32::log(partitions as f32, 2.0); // how far along this pass is in the overall process
        let total_passes = f32::log(image_size as f32 - 1.0, 2.0); // how many total passes will be made
        let rand_scale = 1.0 - (pass_num / total_passes); // reduce variance, the later in the process we are
        real_variance = variance * rand_scale;
    }

    // diamond
    for partition_col in 0..partitions {
        for partition_row in 0..partitions {
            let min_x = partition_col * partition_size;
            let max_x = min_x + partition_size;
            let min_y = partition_row * partition_size;
            let max_y = min_y + partition_size;

            println!("{},{} - {},{}", min_x, min_y, max_x, max_y);

            let mid_x = min_x + (partition_size / 2);
            let mid_y = min_y + (partition_size / 2);

            println!("setting {},{}", mid_x, mid_y);

            let avg = (image.get(min_x, min_y) 
                     + image.get(min_x, max_y) 
                     + image.get(max_x, min_y) 
                     + image.get(max_x, max_y)) / 4.0;
            let val = avg + random_offset(real_variance);

            image.set(mid_x, mid_y, val);
        }
    }

    // square
    let mut visited: HashSet<(i64,i64)> = HashSet::new();
    for partition_col in 0..partitions {
        for partition_row in 0..partitions {
            let min_x = partition_col * partition_size;
            let max_x = min_x + partition_size;
            let min_y = partition_row * partition_size;
            let max_y = min_y + partition_size;
            let mid_x = min_x + (partition_size / 2);
            let mid_y = min_y + (partition_size / 2);

            [
                (min_x, mid_y),
                (mid_x, min_y),
                (mid_x, max_y),
                (max_x, mid_y),
            ].iter()
                .for_each(|&coordinate| {
                    if !visited.contains(&coordinate) {
                        visited.insert(coordinate);

                        let avg = (image.get(coordinate.0 - partition_size / 2, coordinate.1) 
                                + image.get(coordinate.0, coordinate.1 - partition_size / 2) 
                                + image.get(coordinate.0 + partition_size / 2, coordinate.1) 
                                + image.get(coordinate.0, coordinate.1 + partition_size / 2) ) / 4.0;
                        let val = avg + random_offset(real_variance);

                        image.set(coordinate.0, coordinate.1, val);
                    }
                });
        }
    }
}

fn random_offset(range: f32) -> f32 {
    rand::thread_rng().gen_range(-1.0 * range / 2.0, range / 2.0)
}
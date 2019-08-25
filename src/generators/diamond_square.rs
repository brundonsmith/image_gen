
use std::collections::{HashSet};

extern crate rand;
use rand::Rng;

use crate::image::{GrayscaleColor, Image};

/**
 * Creates a new Image and runs apply_diamond_square on it. 
 */
pub fn generate_diamond_square(size: usize, variance: f32, coarseness: f32) -> Image<GrayscaleColor> {
    let mut image = Image::from_color(size, size, 0.5);
    apply_diamond_square(&mut image, variance, coarseness);
    return image;
}

/**
 * Applies the Diamond-Square noise generation algorithm.
 * 
 * Variance is the degree of up/down drift to add to each average; a value of 0 will result
 * in a totally flat map.
 * 
 * Coarseness is a smoothing value; smaller values will make the contours more smooth, higher
 * values will make them more "rough".
 */
pub fn apply_diamond_square(image: &mut Image<GrayscaleColor>, variance: f32, coarseness: f32) {
    assert!(image.width() == image.height(), "Image width and height must be the same");
    assert!((image.width() - 1).is_power_of_two(), "Image width/height must be a power of two plus one");
    assert!(variance >= 0.0, "Variance should be greater than or equal to 0");
    assert!(variance <= 1.0, "Variance should be less than or equal to 1");
    assert!(coarseness >= 0.0, "Coarseness should be greater than or equal to 0");
    assert!(coarseness <= 1.0, "Coarseness should be less than or equal to 1");

    let size = image.width() as i64;
    let mut partitions = 1;
    let mut running_variance = variance;
    while partitions < size - 1 {
        pass(image, partitions, running_variance);
        
        partitions *= 2;
        running_variance *= coarseness;
    }
}

fn pass(image: &mut Image<GrayscaleColor>, partitions: i64, variance: f32) {
    let image_size = image.width() as i64;
    let partition_size = (image_size - 1) / partitions;
    let real_variance = variance * variance_scale(partitions, image_size);

    // diamond
    for partition_col in 0..partitions {
        for partition_row in 0..partitions {
            let part = Partition::from(partition_col, partition_row, partition_size);
            
            let avg = average(part.corners().iter().map(|corner| image.get(corner.0, corner.1)));
            let val = avg + random_offset(real_variance);

            image.set(part.mid_x, part.mid_y, val);
        }
    }

    // square
    let mut visited: HashSet<(i64,i64)> = HashSet::new();
    for partition_col in 0..partitions {
        for partition_row in 0..partitions {
            let part = Partition::from(partition_col, partition_row, partition_size);

            part.sides().iter()
                .for_each(|&coordinate| {
                    if !visited.contains(&coordinate) {
                        visited.insert(coordinate);

                        let avg = average([
                            *image.get(coordinate.0 - partition_size / 2, coordinate.1),
                            *image.get(coordinate.0, coordinate.1 - partition_size / 2),
                            *image.get(coordinate.0 + partition_size / 2, coordinate.1),
                            *image.get(coordinate.0, coordinate.1 + partition_size / 2)
                        ].iter());

                        let val = avg + random_offset(real_variance);

                        image.set(coordinate.0, coordinate.1, val);
                    }
                });
        }
    }
}

fn variance_scale(partitions: i64, image_size: i64) -> f32 {
    let pass_num = f32::log(partitions as f32, 2.0);           // how far along this pass is in the overall process
    let total_passes = f32::log(image_size as f32 - 1.0, 2.0); // how many total passes will be made
    return 1.0 - (pass_num / total_passes);                    // reduce variance, the later in the process we are
}

fn random_offset(range: f32) -> f32 {
    rand::thread_rng().gen_range(-1.0 * range / 2.0, range / 2.0)
}

fn average<'a, I: Clone+Iterator<Item=&'a f32>>(nums: I) -> f32 {
    let count = nums.clone().count() as f32;
    return nums.fold(0.0, |total, &num| total + num) / count;
}

struct Partition {
    pub min_x: i64,
    pub max_x: i64,
    pub min_y: i64,
    pub max_y: i64,
    pub mid_x: i64,
    pub mid_y: i64
}

impl Partition {

    pub fn from(partition_col: i64, partition_row: i64, partition_size: i64) -> Self {
        let min_x = partition_col * partition_size;
        let max_x = min_x + partition_size;
        let min_y = partition_row * partition_size;
        let max_y = min_y + partition_size;
        let mid_x = min_x + (partition_size / 2);
        let mid_y = min_y + (partition_size / 2);

        return Self {
            min_x,
            max_x,
            min_y,
            max_y,
            mid_x,
            mid_y
        };
    }

    pub fn corners(&self) -> [(i64,i64); 4] {
        corners(self.min_x, self.max_x, self.min_y, self.max_y)
    }

    pub fn sides(&self) -> [(i64,i64); 4] {
        [
            (self.min_x, self.mid_y),
            (self.mid_x, self.min_y),
            (self.mid_x, self.max_y),
            (self.max_x, self.mid_y),
        ]
    }
}

fn corners(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> [(i64,i64); 4] {
    [
        (min_x, min_y),
        (min_x, max_y),
        (max_x, min_y),
        (max_x, max_y)
    ]
}

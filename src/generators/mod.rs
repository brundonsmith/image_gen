
pub mod utils;
mod bricks;
mod diamond_square;
mod perlin_noise;
mod hill;

pub use bricks::generate_bricks;
pub use diamond_square::generate_diamond_square;
pub use perlin_noise::{generate_perlin_noise, add_perlin_noise};
pub use hill::{HillShape, generate_hill, add_hill};

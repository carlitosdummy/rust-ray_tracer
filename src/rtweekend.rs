use std::f64::consts::PI;
use rand::Rng;

// Converts degrees to radians using the formula: radians = degrees * π / 180.
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// Generates a random floating-point number between 0.0 (inclusive) and 1.0 (exclusive).
pub fn random_double() -> f64 {
    rand::thread_rng().gen::<f64>()
}

// Generates a random floating-point number within the specified range [min, max).
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

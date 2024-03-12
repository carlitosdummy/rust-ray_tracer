use std::f64::INFINITY;
use std::f64::consts::PI;
use crate::interval;


pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
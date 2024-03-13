#[derive(Debug)]
/// Represents a closed interval [min, max].
pub struct Interval {
    /// The minimum value of the interval.
    pub min: f64,
    /// The maximum value of the interval.
    pub max: f64,
}

impl Interval {
    /// Creates a new `Interval` with the specified minimum and maximum values.
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }
    
    /// Checks if a value is within the interval, inclusive of the endpoints.
    // pub fn contains(&self, x: f64) -> bool {
    //     self.min <= x && x <= self.max
    // }

    /// Checks if a value is strictly inside the interval.
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Clamps a value to be within the interval.
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
}

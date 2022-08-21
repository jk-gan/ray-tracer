use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Self = Self {
        min: f64::MAX,
        max: -f64::MAX,
    };

    pub const UNIVERSE: Self = Self {
        min: -f64::MAX,
        max: f64::MAX,
    };

    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn from_intervals(a: &Interval, b: &Interval) -> Self {
        Self {
            min: f64::min(a.min, b.min),
            max: f64::max(a.max, b.max),
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

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

// default interval is empty
impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::MAX,
            max: -f64::MAX,
        }
    }
}

impl Add<f64> for &Interval {
    type Output = Interval;

    fn add(self, displacement: f64) -> Self::Output {
        Interval::new(self.min + displacement, self.max + displacement)
    }
}

impl Add<&Interval> for f64 {
    type Output = Interval;

    fn add(self, interval: &Interval) -> Self::Output {
        interval + self
    }
}

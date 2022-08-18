pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: -f64::INFINITY,
    };

    const UNIVERSE: Self = Self {
        min: -f64::INFINITY,
        max: f64::INFINITY,
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
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }
}

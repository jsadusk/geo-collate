pub trait Numeric {
    fn half(self) -> Self;
    fn as_float(self) -> f64;
    fn as_int(self) -> i64;
    fn from_float(x: f64) -> Self;
    fn from_int(i: i64) -> Self;
}

impl Numeric for i64 {
    fn half(self) -> i64 {
        self / 2
    }

    fn as_float(self) -> f64 {
        self as f64
    }

    fn as_int(self) -> i64 {
        self
    }

    fn from_float(x: f64) -> Self {
        x as i64
    }

    fn from_int(x: i64) -> Self {
        x
    }
}

impl Numeric for f64 {
    fn half(self) -> f64 {
        self / 2.0
    }

    fn as_float(self) -> f64 {
        self
    }

    fn as_int(self) -> i64 {
        self as i64
    }

    fn from_float(x: f64) -> Self {
        x
    }

    fn from_int(x: i64) -> Self {
        x as f64
    }
}

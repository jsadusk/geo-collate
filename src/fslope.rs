use crate::numeric::*;
use geo::{CoordinateType, Line};

pub trait FSlope {
    fn fslope(&self) -> f64;
    fn inv_fslope(&self) -> f64;
}

impl<T> FSlope for Line<T>
where
    T: CoordinateType + Numeric,
{
    fn fslope(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dy.as_float()) / (dx.as_float())
    }

    fn inv_fslope(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx.as_float()) / (dy.as_float())
    }
}

use std::ops::Shr;
use std::ops::Sub;

pub struct Point {
	pub row: usize,
	pub col: usize,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, p: Point) -> Point {
        let r = Point {
            row:  (self.row as i64 - p.row as i64).abs() as usize,
            col:  (self.col as i64 - p.col as i64).abs() as usize
        };
        return r
    }
}
impl Sub<usize> for Point {
    type Output = Self;

    fn sub(self, s: usize) -> Point {
        let r = Point {
            row:  (self.row as i64 - s as i64).abs() as usize,
            col:  (self.col as i64 - s as i64).abs() as usize
        };
        return r
    }
}

impl Shr<usize> for Point {
    type Output = Self;

    fn shr(self, shr: usize) -> Self {
        Self {row: self.row >> shr, col: self.col >> shr}
    }
}


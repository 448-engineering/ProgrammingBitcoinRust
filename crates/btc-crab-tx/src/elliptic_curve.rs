use crate::{BtcError, BtcResult};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Point {
    a: i64,
    b: i64,
    x: i64,
    y: i64,
}

impl Point {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_x(mut self, x: i64) -> Self {
        self.x = x;

        self
    }

    pub fn add_y(mut self, y: i64) -> Self {
        self.y = y;

        self
    }

    pub fn add_a(mut self, a: i64) -> Self {
        self.a = a;

        self
    }

    pub fn add_b(mut self, b: i64) -> Self {
        self.b = b;

        self
    }

    pub fn build(self) -> BtcResult<Self> {
        if self.y.pow(2) != self.x.pow(3) + (self.a * self.x) + self.b {
            Err(BtcError::NotOnEllipticCurve {
                x: self.x,
                y: self.y,
            })
        } else {
            Ok(self)
        }
    }
}

#[cfg(test)]
mod ec_sanity_checks {
    use crate::Point;

    #[test]
    fn is_on_curve() {
        let point_1 = Point::new().add_x(-1).add_y(-1).add_a(5).add_b(7).build();

        let point_2 = Point::new().add_x(-1).add_y(-2).add_a(5).add_b(7).build();

        assert!(point_1.is_ok());
        assert!(point_2.is_err());
    }
}

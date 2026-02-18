use std::ops;


pub use crate::types::Complex;

impl ops::Add for Complex {

    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

}

impl ops::Sub for Complex {

    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }

}

impl ops::Mul for Complex {

    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x
        }
    }

}

impl ops::Div for Complex {
    
    type Output = Complex;

    fn div(self, rhs: Self) -> Self::Output {
        let denominator = rhs.x * rhs.x + rhs.y * rhs.y;
        Complex {
            x: (self.x * rhs.x + self.y * rhs.y) / denominator,
            y: (self.y * rhs.x - self.x * rhs.y) / denominator
        }
    }
}
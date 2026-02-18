
#![crate_type = "lib"]
#![crate_name = "l0wcomplex"]

mod types;
mod impls;

pub use types::Complex;

impl Complex {

    pub fn abs(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn real(&self) -> f64 {
        self.x
    }

    pub fn imag(&self) -> f64 {
        self.y
    }

    pub fn conjugate(&self) -> Complex {
        Complex {
            x: self.x,
            y: -self.y
        }
    }



}
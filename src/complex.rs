#[derive(Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imag: f64
}

impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag
        }
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Complex;
    fn mul(self, rhs: f64) -> Self::Output {
        Complex {
            real: self.real * rhs,
            imag: self.imag * rhs
        }
    }
}

impl std::ops::Div<f64> for Complex {
    type Output = Complex;
    fn div(self, rhs: f64) -> Self::Output {
        Complex {
            real: self.real / rhs,
            imag: self.imag / rhs
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.imag * rhs.real + self.real * rhs.imag
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Complex;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.conjugate() / rhs.norm_sq()
    }
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }
    pub fn norm_sq(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }
    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }
    pub fn conjugate(&self) -> Complex {
        Complex { real: self.real, imag: -self.imag }
    }
}
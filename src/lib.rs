use num::complex::{Complex, ComplexFloat};

// Möbius transformation: (z + 1) / (z - 1)
pub fn mobius_transformation(z: Complex<f64>) -> Complex<f64> {
    (z + 0.5 * Complex::new(1.0, 0.0)) / (z - 0.23 * Complex::new(1.0, 0.0))
}

// Logarithmic transformation: log(z)
pub fn logarithmic_transformation(z: Complex<f64>) -> Complex<f64> {
    z.ln()
}

// Exponential transformation: exp(z)
pub fn exponential_transformation(z: Complex<f64>) -> Complex<f64> {
    (z * 1.0).exp()
}

// Inverse transformation: 1 / z
pub fn inverse_transformation(z: Complex<f64>) -> Complex<f64> {
    // if z == Complex::zero() {
    //     return Complex::new(f64::INFINITY, f64::INFINITY);
    // }

    Complex::new(1.0, 0.0) / z
}

pub fn black_hole_transformation(z: Complex<f64>) -> Complex<f64> {
    let im = (-1.0 * z.im) - (z.re * z.abs());
    let re = z.re - (z.im * z.abs());

    Complex::new(re, im)
}

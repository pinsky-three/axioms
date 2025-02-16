use num::complex::{Complex, ComplexFloat};

pub struct Transformations;

impl Transformations {
    // Möbius transformation: (z + 1) / (z - 1)
    pub fn mobius_transformation(z: Complex<f64>) -> Complex<f64> {
        (z + 0.5 * Complex::new(1.0, 0.0)) / (z - 0.5 * Complex::new(1.0, 0.0))
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

        Complex::new(1.0, 1.0) / z
    }

    pub fn black_hole_transformation(z: Complex<f64>) -> Complex<f64> {
        let im = -z.im - (z.re * z.abs());
        let re = z.re - (z.im * z.abs());

        Complex::new(re, im)
    }

    pub fn parabolic_transformation(z: Complex<f64>) -> Complex<f64> {
        // let re = z.re + z.im * z.im;
        // let im = z.im;

        // Complex::new(re, im)

        // 1.0 / z + 1.0 / z / z + 1.0 / z / z / z + ...
        (2..7).fold(Complex::new(0.00, 0.00), |acc, term| {
            acc + 1.0 / z.powi(term)
        })
    }

    pub fn sin_transformation(z: Complex<f64>) -> Complex<f64> {
        Complex::new(1.0, 0.0) * (z * Complex::new(2.0, 0.1)).sin()
    }
}

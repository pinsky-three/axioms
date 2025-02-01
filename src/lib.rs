use num::complex::Complex;

// Möbius transformation: (z + 1) / (z - 1)
pub fn mobius_transformation(z: Complex<f64>) -> Complex<f64> {
    (z + Complex::new(1.0, 0.0)) / (z - Complex::new(1.0, 0.0))
}

// Logarithmic transformation: log(z)
pub fn logarithmic_transformation(z: Complex<f64>) -> Complex<f64> {
    z.ln()
}

// Exponential transformation: exp(z)
pub fn exponential_transformation(z: Complex<f64>) -> Complex<f64> {
    z.exp()
}

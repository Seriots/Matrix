
#[cfg(test)]
#[test]
fn linear_interpolation() {
    use assert_float_eq::*;
    use crate::core::Vector;
    use crate::core::Complex;
    use crate::core::linear_interpolation::lerp;
    use crate::Matrix;

    assert_eq!(lerp(10 as i16, 40 as i16, 0.5), 25 as i16);
    assert_eq!(lerp(0., 1., 1.), 1.);
    assert_eq!(lerp(0., 1., 0.5), 0.5);
    assert_f32_near!(lerp(21., 42., 0.3), 27.3);
    assert_eq!(lerp(Vector::from(&[2., 1.]), Vector::from(&[4., 2.]), 0.3), Vector::from(&[2.6, 1.3]));
    assert_eq!(lerp(Vector::from(&[2., 1.]), Vector::from(&[4., 2.]), 0.5), Vector::from(&[3., 1.5]));
    assert_eq!(lerp(Matrix::from(&[&[2., 1.], &[3., 4.]]), Matrix::from(&[&[20., 10.], &[30., 40.]]), 0.5), Matrix::from(&[&[11., 5.5], &[16.5, 22.]]));
    assert_eq!(lerp(Complex::new(1., 2.), Complex::new(3., 4.), 0.5), Complex::new(2., 3.));
    assert_eq!(lerp(Complex::new(1., 2.), Complex::new(3., 4.), 0.3), Complex::new(1.6, 2.6));
    assert_eq!(lerp(Matrix::from(&[&[Complex::new(1., 2.), Complex::new(3., 4.)], &[Complex::new(5., 6.), Complex::new(7., 8.)]]), Matrix::from(&[&[Complex::new(10., 20.), Complex::new(30., 40.)], &[Complex::new(50., 60.), Complex::new(70., 80.)]]), 0.5), Matrix::from(&[&[Complex::new(5.5, 11.), Complex::new(16.5, 22.)], &[Complex::new(27.5, 33.), Complex::new(38.5, 44.)]]));
}

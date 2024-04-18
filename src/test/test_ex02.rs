

#[cfg(test)]
#[test]
fn linear_interpolation() {
    use crate::core::Vector;
    use crate::core::linear_interpolation::lerp;
    use crate::Matrix;

    assert_eq!(lerp(0., 1., 0.), 0.);
    assert_eq!(lerp(0., 1., 1.), 1.);
    assert_eq!(lerp(0., 1., 0.5), 0.5);
    assert_eq!(lerp(21., 42., 0.3), 27.300000250339508);
    assert_eq!(lerp(Vector::from(&[2., 1.]), Vector::from(&[4., 2.]), 0.3), Vector::from(&[2.600000023841858, 1.300000011920929]));
    assert_eq!(lerp(Vector::from(&[2., 1.]), Vector::from(&[4., 2.]), 0.5), Vector::from(&[3., 1.5]));
    assert_eq!(lerp(Matrix::from(&[&[2., 1.], &[3., 4.]]), Matrix::from(&[&[20., 10.], &[30., 40.]]), 0.5), Matrix::from(&[&[11., 5.5], &[16.5, 22.]]));
}
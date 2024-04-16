

#[cfg(test)]
#[test]
fn linear_combination_ex01() {
    use crate::basics::Vector;
    use crate::basics::linear_interpolation::lerp;
    use crate::Matrix;

    assert_eq!(lerp(0., 1., 0.), 0.);
    assert_eq!(lerp(0., 1., 1.), 1.);
    assert_eq!(lerp(0., 1., 0.5), 0.5);
    assert_eq!(lerp(21., 42., 0.3), 27.300000250339508);
    assert_eq!(lerp(Vector::from(vec![2., 1.]), Vector::from(vec![4., 2.]), 0.3).vector, vec![2.600000023841858, 1.300000011920929]);
    assert_eq!(lerp(Vector::from(vec![2., 1.]), Vector::from(vec![4., 2.]), 0.5).vector, vec![3., 1.5]);
    assert_eq!(lerp(Matrix::from(vec![vec![2., 1.], vec![3., 4.]]), Matrix::from(vec![vec![20., 10.], vec![30., 40.]]), 0.5).matrix, vec![vec![11., 5.5], vec![16.5, 22.]]);
}
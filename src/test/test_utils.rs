
#[cfg(test)]
#[test]
fn test_basics_matrix_vector() {
    use crate::basics::Matrix;

    let u = Matrix::from(vec![
        vec![1., 2.],
        vec![3., 4.],
        vec![3., 4.],
    ]);

    let v = u.to_vector();
    let w = v.to_matrix((3, 2));

    let a = Matrix::from(vec![
        vec![1., 2.],
        vec![4., 5.],
    ]);

    assert_eq!(u.shape(), (2, 3));
    assert_eq!(u.is_square(), false);
    assert_eq!(v.size(), 6);
    assert_eq!(w.shape(), (3, 2));
    assert_eq!(w.is_square(), false);
    assert_eq!(a.shape(), (2, 2));
    assert_eq!(a.is_square(), true);

}
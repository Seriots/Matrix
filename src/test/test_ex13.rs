
#[cfg(test)]
#[test]
fn test_rank() {
    use crate::Matrix;

    let u = Matrix::from(&[
        &[1., 0., 0.],
        &[0., 1., 0.],
        &[0., 0., 1.],
    ]);
    assert_eq!(u.rank(), 3);
    // 3
    
    let u = Matrix::from(&[
        &[ 1., 2., 0., 0.],
        &[ 2., 4., 0., 0.],
        &[-1., 2., 1., 1.],
    ]);
    assert_eq!(u.rank(), 2);
    // 2
    
    let u = Matrix::from(&[
        &[ 8., 5., -2.],
        &[ 4., 7., 20.],
        &[ 7., 6., 1.],
        &[21., 18., 7.],
    ]);
    assert_eq!(u.rank(), 3);
    // 3
}
#[cfg(test)]
#[test]
fn test_trace() {
    use crate::Matrix;

    let u = Matrix::from(&[
        &[1., 0.],
        &[0., 1.],
    ]);
    assert_eq!(u.trace(), 2.0);
    // 2.0
    let u = Matrix::from(&[
        &[2., -5., 0.],
        &[4., 3., 7.],
        &[-2., 3., 4.],
    ]);
    assert_eq!(u.trace(), 9.0);

    // 9.0
    let u = Matrix::from(&[
        &[-2., -8., 4.],
        &[1., -23., 4.],
        &[0., 6., 4.],
    ]);
    assert_eq!(u.trace(), -21.0);

    // -21.0
}

#[cfg(test)]
#[test]
#[should_panic]
fn test_trace_panic() {
    use crate::Matrix;

    let u = Matrix::from(&[
        &[1., 0., 1.],
        &[0., 1., 2.],
    ]);
    assert_eq!(u.trace(), 2.0);
}

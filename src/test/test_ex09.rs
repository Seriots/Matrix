
#[cfg(test)]
#[test]
fn test_transpose() {
    use crate::Matrix;

    let u = Matrix::from(&[
        &[1., 0., 0.],
        &[2., 3., 4.],
    ]);
    assert_eq!(u.transpose(), Matrix::from(&[
        &[1., 2.],
        &[0., 3.],
        &[0., 4.],
    ]));

    let u = Matrix::from(&[
        &[1., 2.],
    ]);
    assert_eq!(u.transpose(), Matrix::from(&[
        &[1.],
        &[2.],
    ]));

    let u = Matrix::from(&[
        &[1.],
        &[2.],
    ]);
    assert_eq!(u.transpose(), Matrix::from(&[
        &[1., 2.],
    ]));

    let u = Matrix::from(&[
        &[1., 2.],
        &[1., 2.],
        &[1., 2.],
        &[1., 2.],
        &[1., 2.],
    ]);
    assert_eq!(u.transpose(), Matrix::from(&[
        &[1., 1., 1., 1., 1.],
        &[2., 2., 2., 2., 2.],
    ]));

    let u = Matrix::from(&[
        &[1., 2., 3., 6., 5.],
        &[1., 2., 3., 4., 5.],
        &[1., 8., 3., 4., 5.],
        &[1., 2., 3., 4., 5.],
        &[1., 2., 3., 4., 10.],
    ]);
    assert_eq!(u.transpose(), Matrix::from(&[
        &[1., 1., 1., 1., 1.],
        &[2., 2., 8., 2., 2.],
        &[3., 3., 3., 3., 3.],
        &[6., 4., 4., 4., 4.],
        &[5., 5., 5., 5., 10.],
    ]));
}

#[cfg(test)]
#[test]
fn transpose_complex() {
    use crate::Matrix;
    use crate::Complex;

    let u = Matrix::from(&[
        &[Complex::new(1., 4.), Complex::new(0., 2.), Complex::new(0., -5.)],
        &[Complex::new(2., -3.), Complex::new(3., 12.), Complex::new(4., 4.)],
    ]);

    assert_eq!(u.transpose_complex(), Matrix::from(&[
        &[Complex::new(1., -4.), Complex::new(2., 3.)],
        &[Complex::new(0., -2.), Complex::new(3., -12.)],
        &[Complex::new(0., 5.), Complex::new(4., -4.)],
    ]));

}

#[cfg(test)]
#[test]
fn test_inverse() {
    use crate::Matrix;

    let u = Matrix::from(&[
        &[1., -1., 1.],
        &[2., 3., 0.],
        &[0., -2., 1.],
    ]);
    assert_eq!(u.inverse().unwrap(), Matrix::from(&[
        &[3., -1., -3.],
        &[-2., 1., 2.],
        &[-4., 2., 5.],
    ]));
    let u = Matrix::from(&[
        &[1., 0., 0.],
        &[0., 1., 0.],
        &[0., 0., 1.],
    ]);
    assert_eq!(u.inverse().unwrap(), Matrix::from(&[
        &[1., 0., 0.],
        &[0., 1., 0.],
        &[0., 0., 1.],
    ]));
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let u = Matrix::from(&[
        &[2., 0., 0.],
        &[0., 2., 0.],
        &[0., 0., 2.],
    ]);
    assert_eq!(u.inverse().unwrap(), Matrix::from(&[
        &[0.5, 0., 0.],
        &[0., 0.5, 0.],
        &[0., 0., 0.5],
    ]));
    // [0.5, 0.0, 0.0]
    // [0.0, 0.5, 0.0]
    // [0.0, 0.0, 0.5]
    let u = Matrix::from(&[
        &[8., 5., -2.],
        &[4., 7., 20.],
        &[7., 6., 1.],
    ]);
    assert_eq!(u.inverse().unwrap(), Matrix::from(&[
        &[0.649425287, 0.097701149, -0.655172414],
        &[-0.781609195, -0.126436782, 0.965517241],
        &[0.143678161, 0.074712644, -0.206896552],
    ]));
    // [0.649425287, 0.097701149, -0.655172414]
    // [-0.781609195, -0.126436782, 0.965517241]
    // [0.143678161, 0.074712644, -0.206896552]
}

#[cfg(test)]
#[test]
fn test_inverse_complex() {
    use crate::Matrix;
    use crate::Complex;

    let u = Matrix::from(&[
        &[Complex::new(1., 1.), Complex::new(1., -1.)],
        &[Complex::new(1., 1.), Complex::new(-1., 1.)],
    ]);
    assert_eq!(u.inverse().unwrap(), Matrix::from(&[
        &[Complex::new(0.25, -0.25), Complex::new(0.25, -0.25)],
        &[Complex::new(0.25, 0.25), Complex::new(-0.25, -0.25)],
    ]));
    // [0.0 + 0.5i, 0.0 - 0.5i]
    // [0.5 + 0.0i, -0.5 + 0.0i]
    let u = Matrix::from(&[
        &[Complex::new(4., 5.), Complex::new(3., 2.), Complex::new(-4., -6.)],
        &[Complex::new(12., 9.), Complex::new(14., -1.), Complex::new(10., -4.)],
        &[Complex::new(-13., 1.), Complex::new(-9., 3.), Complex::new(8., 7.)],
    ]);
    assert_eq!(u.inverse().unwrap(), Matrix::from(&[
        &[Complex::new(-0.1164937, -0.34359235), Complex::new(-0.026612714, -0.07645596), Complex::new(0.06578481, -0.23446453)],
        &[Complex::new( -0.02893874, 0.33642697), Complex::new(0.03843019, 0.10525772), Complex::new(-0.19119172, 0.20144512)],
        &[Complex::new(-0.15506421, -0.018762229), Complex::new(0.008555631, -0.02439692), Complex::new(-0.0093443645, -0.082728975)],
    ]));
}

#[cfg(test)]
#[test]
fn vector() {
    use crate::core::Vector;

    //println!("===============Vector Ex00===============");
    let mut u = Vector::from(&[2., 3.]);
    let v = Vector::from(&[5., 7.]);
    u.add(&v);
    assert_eq!(u, Vector::from(&[7., 10.]));

    let mut u = Vector::from(&[2., 3.]);
    let v = Vector::from(&[5., 7.]);
    u.sub(&v);
    assert_eq!(u, Vector::from(&[-3., -4.]));

    let mut u = Vector::from(&[2., 3.]);
    u.scl(2.);
    assert_eq!(u, Vector::from(&[4., 6.]));
}

#[cfg(test)]
#[test]
fn matrix() {
    use crate::core::Matrix;

    //println!("===============Matrix Ex00===============");
    let mut u = Matrix::from(&[
            &[1., 2.],
            &[3., 4.]
        ]);
        let v = Matrix::from(&[
           &[7., 4.],
            &[-2., 2.]
        ]);
        u.add(&v);
        assert_eq!(u, Matrix::from(&[
            &[8., 6.],
            &[1., 6.]
        ]));
        let mut u = Matrix::from(&[
            &[1., 2.],
            &[3., 4.]
        ]);
        let v = Matrix::from(&[
            &[7., 4.],
            &[-2., 2.]
        ]);
        u.sub(&v);
        assert_eq!(u, Matrix::from(&[
            &[-6., -2.],
            &[5., 2.]
        ]));
        let mut u = Matrix::from(&[
            &[1., 2.],
            &[3., 4.]
        ]);
        u.scl(2.);
        assert_eq!(u, Matrix::from(&[
            &[2., 4.],
            &[6., 8.]
        ]));
        // [2.0, 4.0]
        // [6.0, 8.0]
}

#[cfg(test)]
#[test]
fn vector_complex() {
    use crate::core::Vector;
    use crate::core::Complex;

    let mut a = Vector::from(&[Complex::new(1., 2.), Complex::new(3., 4.)]);
    let b = Vector::from(&[Complex::new(5., 6.), Complex::new(7., 8.)]);
    assert_eq!(a, Vector::from(&[Complex::new(1., 2.), Complex::new(3., 4.)]));
    a.add(&b);
    assert_eq!(a, Vector::from(&[Complex::new(6., 8.), Complex::new(10., 12.)]));

    let mut a = Vector::from(&[Complex::new(1., 2.), Complex::new(3., 4.)]);
    a.sub(&b);

    assert_eq!(a, Vector::from(&[Complex::new(-4., -4.), Complex::new(-4., -4.)]));

    let mut a = Vector::from(&[Complex::new(1., 2.), Complex::new(3., 4.)]);

    a.scl(Complex::new(2., 0.));
    assert_eq!(a, Vector::from(&[Complex::new(2., 4.), Complex::new(6., 8.)]));
}

#[cfg(test)]
#[test]
fn matrix_complex() {
    use crate::core::Matrix;
    use crate::core::Complex;

    let mut a = Matrix::from(&[
        &[Complex::new(1., 2.), Complex::new(3., 4.)],
        &[Complex::new(5., 6.), Complex::new(7., 8.)]
    ]);

    let b = Matrix::from(&[
        &[Complex::new(9., 10.), Complex::new(11., 12.)],
        &[Complex::new(13., 14.), Complex::new(15., 16.)]
    ]);

    a.add(&b);

    assert_eq!(a, Matrix::from(&[
        &[Complex::new(10., 12.), Complex::new(14., 16.)],
        &[Complex::new(18., 20.), Complex::new(22., 24.)]
    ]));

    let mut a = Matrix::from(&[
        &[Complex::new(1., 2.), Complex::new(3., 4.)],
        &[Complex::new(5., 6.), Complex::new(7., 8.)]
    ]);

    a.sub(&b);

    assert_eq!(a, Matrix::from(&[
        &[Complex::new(-8., -8.), Complex::new(-8., -8.)],
        &[Complex::new(-8., -8.), Complex::new(-8., -8.)]
    ]));

    let mut a = Matrix::from(&[
        &[Complex::new(1., 2.), Complex::new(3., 4.)],
        &[Complex::new(5., 6.), Complex::new(7., 8.)]
    ]);

    a.scl(Complex::new(2., 0.));

    assert_eq!(a, Matrix::from(&[
        &[Complex::new(2., 4.), Complex::new(6., 8.)],
        &[Complex::new(10., 12.), Complex::new(14., 16.)]
    ]));
}
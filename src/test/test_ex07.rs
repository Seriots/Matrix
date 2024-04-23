
#[cfg(test)]
#[test]
fn matrix_multiplication() {
    use crate::{Matrix, Vector};

    let u = Matrix::from(&[
        &[1., 0.],
        &[0., 1.],
        ]);
    let v = Vector::from(&[4., 2.]);
    assert_eq!(u.mul_vec(&v), Vector::from(&[4., 2.]));
    // [4.]
    // [2.]

    let u = Matrix::from(&[
    &[11., 3.],
    &[7., 1.],
    &[8., 5.],
    ]);
    let v = Vector::from(&[5., 6.]);
    assert_eq!(u.mul_vec(&v), Vector::from(&[73., 41., 70.]));

    let u = Matrix::from(&[
    &[2., 0.],
    &[0., 2.],
    ]);
    let v = Vector::from(&[4., 2.]);
    assert_eq!(u.mul_vec(&v), Vector::from(&[8., 4.]));

    let u = Matrix::from(&[
    &[2., -2.],
    &[-2., 2.],
    ]);
    let v = Vector::from(&[4., 2.]);
    assert_eq!(u.mul_vec(&v), Vector::from(&[4., -4.]));

    let u = Matrix::from(&[
        &[2, -2],
        &[-2, 2],
    ]);
    let v = Vector::from(&[4, 2]);
    assert_eq!(u.mul_vec(&v), Vector::from(&[4, -4]));

    let u = Matrix::from(&[
        &[1., 0.],
        &[0., 1.],
    ]);
    let v = Matrix::from(&[
        &[1., 0.],
        &[0., 1.],
    ]);
    assert_eq!(u.mul_mat(&v), Matrix::from(&[
        &[1., 0.],
        &[0., 1.],
        ]));


    let u = Matrix::from(&[
        &[1., 0.],
        &[0., 1.],
    ]);
    let v = Matrix::from(&[
        &[2., 1.],
        &[4., 2.],
    ]);
    assert_eq!(u.mul_mat(&v), Matrix::from(&[
        &[2., 1.],
        &[4., 2.],
    ]));


    let u = Matrix::from(&[
        &[3., -5.],
        &[6., 8.],
    ]);
    let v = Matrix::from(&[
        &[2., 1.],
        &[4., 2.],
    ]);
    assert_eq!(u.mul_mat(&v), Matrix::from(&[
        &[-14., -7.],
        &[44., 22.],
    ]));

    let u = Matrix::from(&[
        &[1., 0.],
        &[0., 1.],
        &[0., 1.],
    ]);
    let v = Matrix::from(&[
        &[2., 1., 5., 7.],
        &[4., 2., 8., 6.],
    ]);
    assert_eq!(u.mul_mat(&v), Matrix::from(&[
        &[2., 1., 5., 7.],
        &[4., 2., 8., 6.],
        &[4., 2., 8., 6.],
        ]));

}

#[cfg(test)]
#[test]
#[should_panic]
fn matrix_multiplication_panic() {
    use crate::Matrix;

    let u = Matrix::from(&[
        &[1., 0., 2.],
        &[0., 1., 2.],
        &[0., 1., 2.],
        ]);
    let v = Matrix::from(&[
    &[2., 1., 5., 7.],
    &[4., 2., 8., 6.],
    ]);
    assert_eq!(u.mul_mat(&v), Matrix::from(&[
        &[2., 1., 5., 7.],
        &[4., 2., 8., 6.],
        &[4., 2., 8., 6.],
        ]));
    // [2., 1.]
    // [4., 2.]
}

#[cfg(test)]
#[test]
fn matrix_multiplication_complex() {
    use crate::{Matrix, Vector, Complex};

    let u = Matrix::from(&[
        &[Complex::new(1., 0.), Complex::new(0., 0.)],
        &[Complex::new(0., 0.), Complex::new(1., 0.)],
        ]);
    let v = Vector::from(&[Complex::new(4., 3.), Complex::new(2., 1.)]);
    assert_eq!(u.mul_vec(&v), Vector::from(&[Complex::new(4., 3.), Complex::new(2., 1.)]));

    let u = Matrix::from(&[
        &[Complex::new(11., 3.), Complex::new(7., 1.)],
        &[Complex::new(8., 5.), Complex::new(2., 1.)],
        ]);

    let v = Vector::from(&[Complex::new(5., 6.), Complex::new(4., 3.)]);

    assert_eq!(u.mul_vec(&v), Vector::from(&[Complex::new(62., 106.), Complex::new(15., 83.)]));

    let u = Matrix::from(&[
        &[Complex::new(2., 0.), Complex::new(0., 0.)],
        &[Complex::new(0., 0.), Complex::new(2., 0.)],
        ]);

    let v = Matrix::from(&[
        &[Complex::new(4., 3.), Complex::new(2., 1.)],
        &[Complex::new(4., 3.), Complex::new(2., 1.)],
        ]);

    assert_eq!(u.mul_mat(&v), Matrix::from(&[
        &[Complex::new(8., 6.), Complex::new(4., 2.)],
        &[Complex::new(8., 6.), Complex::new(4., 2.)],
        ]));


}
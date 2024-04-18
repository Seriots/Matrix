
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
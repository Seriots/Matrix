
#[cfg(test)]
#[test]
fn dot_product() {
    use crate::Vector;

    let u = Vector::from(&[0., 0.]);
    let v = Vector::from(&[1., 1.]);
    assert_eq!(u.dot(v), 0.);
    // 0.0
    let u = Vector::from(&[1., 1.]);
    let v = Vector::from(&[1., 1.]);
    assert_eq!(u.dot(v), 2.);
    // 2.0
    let u = Vector::from(&[-1., 6.]);
    let v = Vector::from(&[3., 2.]);
    assert_eq!(u.dot(v), 9.);
    // 9.0
}

#[cfg(test)]
#[test]
fn dot_product_complex() {
    use crate::Vector;
    use crate::Complex;

    let u = Vector::from(&[Complex::new(0., 1.), Complex::new(0., 1.)]);
    let v = Vector::from(&[Complex::new(1., 1.), Complex::new(1., 1.)]);
    assert_eq!(u.dot(v), Complex::new(-2., 2.));

    let u = Vector::from(&[Complex::new(1., 1.), Complex::new(1., 1.)]);
    let v = Vector::from(&[Complex::new(1., 1.), Complex::new(1., 1.)]);
    assert_eq!(u.dot(v), Complex::new(0., 4.));

    let u = Vector::from(&[-Complex::new(1., 1.), Complex::new(6., 1.)]);
    let v = Vector::from(&[Complex::new(3., 1.), Complex::new(2., 1.)]);
    assert_eq!(u.dot(v), Complex::new(9., 4.));

    let u = Vector::from(&[-Complex::new(1., 4.), Complex::new(6., 3.)]);
    let v = Vector::from(&[Complex::new(3., -2.), Complex::new(-2., 5.)]);
    assert_eq!(u.dot(v), Complex::new(-38., 14.));
}
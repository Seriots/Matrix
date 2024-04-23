#[cfg(test)]
#[test]
fn test_cross_product() {
    use crate::Vector;

    let u = Vector::from(&[0., 0., 1.]);
    let v = Vector::from(&[1., 0., 0.]);
    assert_eq!(Vector::cross_product(&u, &v), Vector::from(&[0., 1., 0.]));
    let u = Vector::from(&[1., 2., 3.]);
    let v = Vector::from(&[4., 5., 6.]);
    assert_eq!(Vector::cross_product(&u, &v), Vector::from(&[-3., 6., -3.]));
    let u = Vector::from(&[4., 2., -3.]);
    let v = Vector::from(&[-2., -5., 16.]);
    assert_eq!(Vector::cross_product(&u, &v), Vector::from(&[17., -58., -16.]));
}

#[cfg(test)]
#[test]
fn test_cross_product_complex() {
    use crate::Vector;
    use crate::Complex;

    let u = Vector::from(&[Complex::new(0., 1.), Complex::new(0., 1.), Complex::new(1., 1.)]);
    let v = Vector::from(&[Complex::new(1., 1.), Complex::new(0., 1.), Complex::new(0., 1.)]);
    assert_eq!(Vector::cross_product(&u, &v), Vector::from(&[Complex::new(0., -1.), Complex::new(1., 2.), Complex::new(0., -1.)]));

    let u = Vector::from(&[Complex::new(1., 1.), Complex::new(2., 1.), Complex::new(3., 1.)]);
    let v = Vector::from(&[Complex::new(4., 1.), Complex::new(5., 1.), Complex::new(6., 1.)]);
    assert_eq!(Vector::cross_product(&u, &v), Vector::from(&[Complex::new(-3., 0.), Complex::new(6., 0.), Complex::new(-3., 0.)]));
    
    let u = Vector::from(&[Complex::new(4., 2.), Complex::new(2., 1.), Complex::new(-3., 1.)]);
    let v = Vector::from(&[Complex::new(-2., 1.), Complex::new(-5., 1.), Complex::new(16., 1.)]);
    assert_eq!(Vector::cross_product(&u, &v), Vector::from(&[Complex::new(17., 26.), Complex::new(-57., -41.), Complex::new(-17., -6.)]));
}
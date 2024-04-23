#[cfg(test)]
#[test]
fn test_cosinus() {
    use assert_float_eq::*;

    use crate::Vector;

    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[1., 0.]);
    assert_eq!(Vector::angle_cos(&u, &v), 1.0);
    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[0., 1.]);
    assert_eq!(Vector::angle_cos(&u, &v), 0.0);
    let u = Vector::from(&[-1., 1.]);
    let v = Vector::from(&[ 1., -1.]);
    assert_f32_near!(Vector::angle_cos(&u, &v), -1.);
    let u = Vector::from(&[2., 1.]);
    let v = Vector::from(&[4., 2.]);
    assert_f32_near!(Vector::angle_cos(&u, &v), 1.0);
    let u = Vector::from(&[1., 2., 3.]);
    let v = Vector::from(&[4., 5., 6.]);
    assert_f32_near!(Vector::angle_cos(&u, &v), 0.9746318);
}

#[cfg(test)]
#[test]
fn test_cosinus_complex() {

    use crate::Vector;
    use crate::Complex;

    let u = Vector::from(&[Complex::new(1., 1.), Complex::new(0., 1.)]);
    let v = Vector::from(&[Complex::new(1., 1.), Complex::new(0., 1.)]);
    assert_eq!(Vector::angle_cos(&u, &v), Complex::new(-0.6, -0.8));
    let u = Vector::from(&[Complex::new(1., 1.), Complex::new(0., 1.)]);
    let v = Vector::from(&[Complex::new(0., 1.), Complex::new(1., 1.)]);
    assert_eq!(Vector::angle_cos(&u, &v), Complex::new(-0.4, -1.2));
    let u = Vector::from(&[-Complex::new(1., 1.), Complex::new(1., 1.)]);
    let v = Vector::from(&[ Complex::new(1., 1.), -Complex::new(1., 1.)]);
    assert_eq!(Vector::angle_cos(&u, &v), Complex::new(-1., 0.));
    let u = Vector::from(&[Complex::new(2., 1.), Complex::new(1., 1.)]);
    let v = Vector::from(&[Complex::new(4., 1.), Complex::new(2., 1.)]);
    assert_eq!(Vector::angle_cos(&u, &v), Complex::new(0.99957836, -0.0034203322));
    let u = Vector::from(&[Complex::new(1., 1.), Complex::new(2., 1.), Complex::new(3., 1.)]);
    let v = Vector::from(&[Complex::new(4., 1.), Complex::new(5., 1.), Complex::new(6., 1.)]);
    assert_eq!(Vector::angle_cos(&u, &v), Complex::new(0.99291223, 0.01960226));
}

#[cfg(test)]
#[test]
fn norm() {
    use crate::Vector;
    use assert_float_eq::*;

    let u = Vector::from(&[0., 0., 0.]);
    assert_eq!(u.norm_1(), 0.);
    assert_eq!(u.norm(), 0.);
    assert_eq!(u.norm_inf(), 0.);

    let u = Vector::from(&[1., 2., 3.]);
    assert_eq!(u.norm_1(), 6.);
    assert_f32_near!(u.norm(), 3.7416575);
    assert_eq!(u.norm_inf(), 3.);
    // 6.0, 3.74165738, 3.0

    let u = Vector::from(&[-1., -2.]);
    assert_eq!(u.norm_1(), 3.);
    assert_f32_near!(u.norm(), 2.236068);
    assert_eq!(u.norm_inf(), 2.);
    // 3.0, 2.236067977, 2.0
}

#[cfg(test)]
#[test]

fn norm_complex() {
    use crate::Vector;
    use crate::Complex;
    use assert_float_eq::*;

    let u = Vector::from(&[Complex::new(0., 1.), Complex::new(0., 1.), Complex::new(0., 1.)]);
    assert_eq!(u.norm_1(), Complex::new(0., 3.));
    assert_eq!(u.norm(), Complex::new(0., 1.7320508));
    assert_eq!(u.norm_inf(), Complex::new(0., 1.));

    let u = Vector::from(&[Complex::new(1., 1.), Complex::new(2., 1.), Complex::new(3., 1.)]);
    assert_eq!(u.norm_1(), Complex::new(6., 3.));
    assert_eq!(u.norm(), Complex::new(3.693157, 1.6246262));
    assert_eq!(u.norm_inf(), Complex::new(3., 1.));
    // 6.0, 3.74165738, 3.0
    
    let u = Vector::from(&[-Complex::new(1., 1.), -Complex::new(2., 1.)]);
    assert_eq!(u.norm_1(), Complex::new(3., 2.));
    assert_eq!(u.norm(), Complex::new(2.2032025, 1.3616542));
    assert_eq!(u.norm_inf(), Complex::new(2., 1.));
    // 3.0, 2.236067977, 2.0
}
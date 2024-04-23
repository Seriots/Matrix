
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
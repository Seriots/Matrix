#[cfg(test)]
#[test]
fn linear_combination() {
    use crate::core::Vector;
    use crate::core::linear_combination;

    let e1 = Vector::from(&[1., 0., 0.]);
    let e2 = Vector::from(&[0., 1., 0.]);
    let e3 = Vector::from(&[0., 0., 1.]);
    let v1 = Vector::from(&[1., 2., 3.]);
    let v2 = Vector::from(&[0., 10., -100.]);
    let test1 = linear_combination::<f32>(&[e1, e2, e3], &[10., -2., 0.5]);
    let test2 = linear_combination::<f32>(&[v1, v2], &[10., -2.]);
    
    assert_eq!(test1, Vector::from(&[10., -2., 0.5]));
    assert_eq!(test2, Vector::from(&[10., 0., 230.]));
}

#[cfg(test)]
#[test]
fn linear_combination_complex() {
    use crate::core::Vector;
    use crate::core::Complex;
    use crate::core::linear_combination;

    let e1 = Vector::from(&[Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.)]);
    let e2 = Vector::from(&[Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.)]);
    let e3 = Vector::from(&[Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.)]);
    let v1 = Vector::from(&[Complex::new(1., 2.), Complex::new(3., 4.), Complex::new(5., 6.)]);
    let v2 = Vector::from(&[Complex::new(0., 10.), Complex::new(-100., 0.), Complex::new(0., 0.)]);
    let test1 = linear_combination::<Complex>(&[e1, e2, e3], &[Complex::new(10., 0.), Complex::new(-2., 0.), Complex::new(0.5, 0.)]);
    let test2 = linear_combination::<Complex>(&[v1.clone(), v2.clone()], &[Complex::new(10., 0.), Complex::new(-2., 0.)]);
    let test3 = linear_combination::<Complex>(&[v1, v2], &[Complex::new(10., 4.), Complex::new(-2., -8.)]);
    
    assert_eq!(test1, Vector::from(&[Complex::new(10., 0.), Complex::new(-2., 0.), Complex::new(0.5, 0.)]));
    assert_eq!(test2, Vector::from(&[Complex::new(10., 0.), Complex::new(230., 40.), Complex::new(50., 60.)]));
    assert_eq!(test3, Vector::from(&[Complex::new(82., 4.), Complex::new(214., 852.), Complex::new(26., 80.)]));
}
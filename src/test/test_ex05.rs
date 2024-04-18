#[cfg(test)]
#[test]
fn test_cosinus() {
    use crate::Vector;

    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[1., 0.]);
    assert_eq!(Vector::angle_cos(&u, &v), 1.0);
    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[0., 1.]);
    assert_eq!(Vector::angle_cos(&u, &v), 0.0);
    let u = Vector::from(&[-1., 1.]);
    let v = Vector::from(&[ 1., -1.]);
    assert_eq!(Vector::angle_cos(&u, &v), -1.0000001);
    let u = Vector::from(&[2., 1.]);
    let v = Vector::from(&[4., 2.]);
    assert_eq!(Vector::angle_cos(&u, &v), 1.0);
    let u = Vector::from(&[1 as usize, 2, 3]);
    let v = Vector::from(&[4, 5, 6]);
    assert_eq!(Vector::angle_cos(&u, &v), 0.9746318);
}
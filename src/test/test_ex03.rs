
#[cfg(test)]
#[test]
fn dot_product() {
    use crate::Vector;

    let u = Vector::from(vec![0., 0.]);
    let v = Vector::from(vec![1., 1.]);
    assert_eq!(u.dot(v), 0.);
    // 0.0
    let u = Vector::from(vec![1., 1.]);
    let v = Vector::from(vec![1., 1.]);
    assert_eq!(u.dot(v), 2.);
    // 2.0
    let u = Vector::from(vec![-1., 6.]);
    let v = Vector::from(vec![3., 2.]);
    assert_eq!(u.dot(v), 9.);
    // 9.0
}
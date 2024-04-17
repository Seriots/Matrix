#[cfg(test)]
#[test]
fn test_cross_product() {
    use crate::Vector;

    let u = Vector::from(vec![0., 0., 1.]);
    let v = Vector::from(vec![1., 0., 0.]);
    assert_eq!(Vector::cross_product(&u, &v).vector,vec![0., 1., 0.]);
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    assert_eq!(Vector::cross_product(&u, &v).vector,vec![-3., 6., -3.] );
    let u = Vector::from(vec![4., 2., -3.]);
    let v = Vector::from(vec![-2., -5., 16.]);
    assert_eq!(Vector::cross_product(&u, &v).vector,vec![17., -58., -16.] );
}
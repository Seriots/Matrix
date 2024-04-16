#[cfg(test)]
#[test]
fn linear_combination_ex01() {
    use crate::basics::Vector;
    use crate::basics::linear_combination;

    let e1 = Vector::from(vec![1., 0., 0.]);
    let e2 = Vector::from(vec![0., 1., 0.]);
    let e3 = Vector::from(vec![0., 0., 1.]);
    let v1 = Vector::from(vec![1., 2., 3.]);
    let v2 = Vector::from(vec![0., 10., -100.]);
    let test1 = linear_combination::<f32>(&[e1, e2, e3], &[10., -2., 0.5]);
    let test2 = linear_combination::<f32>(&[v1, v2], &[10., -2.]);
    
    assert_eq!(test1.vector, vec![10., -2., 0.5]);
    assert_eq!(test2.vector, vec![10., 0., 230.]);
}
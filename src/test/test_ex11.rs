
#[cfg(test)]
#[test]
fn test_determinant() {
    use crate::Matrix;

	let mat = Matrix::from(&[
		&[8., 5.],
		&[6., 2.5],
	]);
	assert_eq!(mat.determinant(), -10.0);
	
	let mat = Matrix::from(&[
		&[8., 5., -2.],
		&[4., 2.5, 20.],
		&[7., 5., 1.],
	]);
	assert_eq!(mat.determinant(), -105.0);
	
	let mat = Matrix::from(&[
		&[8., 5., -2., 7.],
		&[4., 2.5, 20., 18.],
		&[7., 5., 1., 4.],
		&[10., 8., 2., 9.],
	]);
	assert_eq!(mat.determinant(), -883.0);
	
	let u = Matrix::from(&[
		&[ 1., -1.],
		&[-1., 1.],
	]);
	assert_eq!(u.determinant(), 0.0);
	// 0.0

	let u = Matrix::from(&[
		&[2., 0., 0.],
		&[0., 2., 0.],
		&[0., 0., 2.],
	]);
	assert_eq!(u.determinant(), 8.0);
	// 8.0
	
	let u = Matrix::from(&[
		&[8., 5., -2.],
		&[4., 7., 20.],
		&[7., 6., 1.],
	]);
	assert_eq!(u.determinant(), -174.0);
	// -174.0
	
	let u = Matrix::from(&[
		&[ 8., 5., -2., 4.],
		&[ 4., 2.5, 20., 4.],
		&[ 8., 5., 1., 4.],
		&[28., -4., 17., 1.],
	]);
	assert_eq!(u.determinant(), 1032.0);
	// 1032
}

#[cfg(test)]
#[test]
#[should_panic]
fn test_determinant_panic1() {
	use crate::Matrix;

	let u = Matrix::from(&[
		&[8., 5., -2., 4.],
		&[4., 7., 20., 5.],
		&[7., 6., 1., 6.],
	]);
	assert_eq!(u.determinant(), -174.0);
}


#[cfg(test)]
#[test]
#[should_panic]
fn test_determinant_panic2() {
	use crate::Matrix;

	let u = Matrix::from(&[
		&[8., 5., -2., 4., 7.],
		&[4., 7., 20., 5., 10.],
		&[7., 6., 1., 6., 13.],
		&[8., 5., -2., 4., 7.],
		&[4., 7., 20., 5., 10.],
	]);
	assert_eq!(u.determinant(), -174.0);
}

#[cfg(test)]
#[test]
fn test_determinant_complex() {
	use crate::Matrix;
	use crate::Complex;

	let mat = Matrix::from(&[
		&[Complex::new(8., 2.), Complex::new(5., -3.)],
		&[Complex::new(6., 1.), Complex::new(2.5, 1.)],
	]);
	assert_eq!(mat.determinant(), Complex::new(-15., 26.));

	let mat = Matrix::from(&[
		&[Complex::new(8., 2.), Complex::new(5., -3.), Complex::new(-2., 1.)],
		&[Complex::new(4., 1.), Complex::new(2.5, 1.), Complex::new(20., 1.)],
		&[Complex::new(7., 1.), Complex::new(5., 1.), Complex::new(1., 1.)],
	]);

	assert_eq!(mat.determinant(), Complex::new(4.5, -661.5));

	let mat = Matrix::from(&[
		&[Complex::new(8., 2.), Complex::new(5., -3.), Complex::new(-2., 1.), Complex::new(7., 1.)],
		&[Complex::new(4., 1.), Complex::new(2.5, 1.), Complex::new(20., 1.), Complex::new(18., 1.)],
		&[Complex::new(7., 1.), Complex::new(5., 1.), Complex::new(1., 1.), Complex::new(4., 1.)],
		&[Complex::new(10., 1.), Complex::new(8., 1.), Complex::new(2., 1.), Complex::new(9., 1.)],
	]);

	assert_eq!(mat.determinant(), Complex::new(-495., -1833.5));

}
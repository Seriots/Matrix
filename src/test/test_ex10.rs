
#[cfg(test)]
#[test]
fn test_row_echelon_form() {
    use crate::Matrix;

    let u = Matrix::from(&[
			&[1., 0., 0.],
			&[0., 1., 0.],
			&[0., 0., 1.],
		]);
		assert_eq!(u.row_echelon(), Matrix::from(&[
			&[1., 0., 0.],
			&[0., 1., 0.],
			&[0., 0., 1.],
		]));

		// [1.0, 0.0, 0.0]
		// [0.0, 1.0, 0.0]
		// [0.0, 0.0, 1.0]
		let u = Matrix::from(&[
			&[1., 2.],
			&[3., 4.],
		]);
		assert_eq!(u.row_echelon(), Matrix::from(&[
			&[1., 0.],
			&[0., 1.],
		]));

		// [1.0, 0.0]
		// [0.0, 1.0]
		let u = Matrix::from(&[
			&[1., 2.],
			&[2., 4.],
		]);
		assert_eq!(u.row_echelon(), Matrix::from(&[
			&[1., 2.],
			&[0., 0.],
		]));
		// [1.0, 2.0]
		// [0.0, 0.0]
		let u = Matrix::from(&[
			&[8., 5., -2., 4., 28.],
			&[4., 2.5, 20., 4., -4.],
			&[8., 5., 1., 4., 17.],
		]);
		assert_eq!(u.row_echelon(), Matrix::from(&[
			&[1., 0.625, 0., 0., -12.1666667],
			&[0., 0., 1., 0., -3.6666667],
			&[0., 0., 0., 1., 29.5],
		]));
		// [1.0, 0.625, 0.0, 0.0, -12.1666667]
		// [0.0, 0.0, 1.0, 0.0, -3.6666667]
		// [0.0, 0.0, 0.0, 1.0, 29.5 ]
}

#[cfg(test)]
#[test]
fn test_row_echelon_form_complex() {
	use crate::Matrix;
	use crate::Complex;

	let u = Matrix::from(&[
			&[Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.)],
			&[Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.)],
			&[Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.)],
		]);
		assert_eq!(u.row_echelon(), Matrix::from(&[
			&[Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.)],
			&[Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.)],
			&[Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.)],
		]));

		// [1.0, 0.0, 0.0]
		// [0.0, 1.0, 0.0]
		// [0.0, 0.0, 1.0]
		let u = Matrix::from(&[
			&[Complex::new(1., 2.), Complex::new(2., -4.)],
			&[Complex::new(3., -1.), Complex::new(4., 12.)],
		]);
		assert_eq!(u.row_echelon(), Matrix::from(&[
			&[Complex::new(1., 0.), Complex::new(0., 0.)],
			&[Complex::new(0., 0.), Complex::new(1., 0.)],
		]));

		let u = Matrix::from(&[
			&[Complex::new(1., 2.), Complex::new(2., -4.), Complex::new(3., 1.)],
			&[Complex::new(3., -1.), Complex::new(4., 12.), Complex::new(5., 2.)],
		]);
		assert_eq!(u.row_echelon(), Matrix::from(&[
			&[Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(1.4024391,  -0.37804872)],
			&[Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0.3695122, 0.025609758)],
		]));

}
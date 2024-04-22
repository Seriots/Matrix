
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use num_traits::pow;

use crate::utils::DefaultOne;
use crate::utils::Fma;
use crate::utils::IntoF32;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
	real: f32,
	imag: f32,
}

impl Complex {
	pub fn new(real: f32, imag: f32) -> Self {
		Self { real, imag }
	}
}

impl Display for Complex {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{} + {}i", self.real, self.imag)
	}
}

impl DefaultOne for Complex {
	fn one() -> Self {
		Self::new(1.0, 0.0)
	}
}

impl Default for Complex {
	fn default() -> Self {
		Self::new(0.0, 0.0)
	}
}

impl Add for Complex {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Self::new(self.real + rhs.real, self.imag + rhs.imag)
	}
}

impl AddAssign for Complex {
	fn add_assign(&mut self, rhs: Self) {
		self.real += rhs.real;
		self.imag += rhs.imag;
	}
}

impl Sub for Complex {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		Self::new(self.real - rhs.real, self.imag - rhs.imag)
	}
}

impl SubAssign for Complex {
	fn sub_assign(&mut self, rhs: Self) {
		self.real -= rhs.real;
		self.imag -= rhs.imag;
	}
}

impl Mul for Complex {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		Self::new(
			self.real * rhs.real - self.imag * rhs.imag,
			self.real * rhs.imag + self.imag * rhs.real,
		)
	}
}

impl MulAssign for Complex {
	fn mul_assign(&mut self, rhs: Self) {
		*self = Self::new(
			self.real * rhs.real - self.imag * rhs.imag,
			self.real * rhs.imag + self.imag * rhs.real,
		);
	}
}

impl Div for Complex {
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		let denom = rhs.real * rhs.real + rhs.imag * rhs.imag;
		Self::new(
			(self.real * rhs.real + self.imag * rhs.imag) / denom,
			(self.imag * rhs.real - self.real * rhs.imag) / denom,
		)
	}
}

impl DivAssign for Complex {
	fn div_assign(&mut self, rhs: Self) {
		let denom = rhs.real * rhs.real + rhs.imag * rhs.imag;
		*self = Self::new(
			(self.real * rhs.real + self.imag * rhs.imag) / denom,
			(self.imag * rhs.real - self.real * rhs.imag) / denom,
		);
	}
}

impl Neg for Complex {
	type Output = Self;

	fn neg(self) -> Self {
		Self::new(-self.real, -self.imag)
	}
}

impl Fma for Complex {
	fn fma(a: Self, b: Self, c: Self) -> Self {
		a * b + c
	}

	fn sfma(&mut self, a: Self, b: Self) {
		*self += a * b;
	}
}

impl IntoF32 for Complex {
	fn into_f32(self) -> f32 {
		(self.real * self.real + self.imag * self.imag).sqrt()
	}
}
//Debug + Display + DefaultOne + Clone + Default + Fma + IntoF32 + AddAssign + Add<Output = K> + SubAssign + Sub<Output = K> + MulAssign + DivAssign + Mul<Output = K> + Div<Output = K> + Neg<Output = K> + PartialEq
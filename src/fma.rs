// Module: fma
// fma => a * b + c
// sfma => a * b + self to self
pub trait Fma {
	fn fma(a: Self, b: Self, c: Self) -> Self;
	fn sfma(&mut self, a: Self, b: Self);
}

impl Fma for f32 {
	fn fma(a: f32, b: f32, c: f32) -> f32 {
		a.mul_add(b, c)
	}

	fn sfma(&mut self, a: f32, b: f32) {
		*self = a.mul_add(b, *self);
	}
}

impl Fma for f64 {
	fn fma(a: f64, b: f64, c:f64) -> f64 {
		a.mul_add(b, c)
	}
	fn sfma(&mut self, a: Self, b: Self) {
		*self = a.mul_add(b, *self);
	}
}

impl Fma for i32 {
	fn fma(a: i32, b: i32, c: i32) -> i32 {
		a.wrapping_mul(b).wrapping_add(c)
	}
	
	fn sfma(&mut self, a: Self, b: Self) {
		*self = a.wrapping_mul(b).wrapping_add(*self);
	}
}
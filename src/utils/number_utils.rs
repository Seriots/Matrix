
pub trait NumberUtils {
	fn one() -> Self;
	fn approx_zero(&self) -> bool;
	fn absolute(&self) -> Self;
	fn power(&self, n: f32) -> Self;
}

impl NumberUtils for f32 {
	fn one() -> Self {
		1.0
	}

	fn approx_zero(&self) -> bool {
		(self).abs() < 1e-6
	
	}

	fn absolute(&self) -> Self {
		self.abs()
	}

	fn power(&self, n: f32) -> Self {
		self.powf(n)
	}
}

impl NumberUtils for f64 {
	fn one() -> Self {
		1.0
	}

	fn approx_zero(&self) -> bool {
		(self).abs() < 1e-6
	}

	fn absolute(&self) -> Self {
		self.abs()
	}

	fn power(&self, n: f32) -> Self {
		self.powf(n as f64)
	}
}

impl NumberUtils for i8 {
	fn one() -> Self {
		1
	}

	fn approx_zero(&self) -> bool {
		(self).abs() == 0
	}

	fn absolute(&self) -> Self {
		self.abs()
	}

	fn power(&self, n: f32) -> Self {
		self.pow(n as u32)
	}
}

impl NumberUtils for i16 {
	fn one() -> Self {
		1
	}

	fn approx_zero(&self) -> bool {
		(self).abs() == 0
	}

	fn absolute(&self) -> Self {
		self.abs()
	}

	fn power(&self, n: f32) -> Self {
		self.pow(n as u32)
	}
}

impl NumberUtils for i32 {
	fn one() -> Self {
		1
	}

	fn approx_zero(&self) -> bool {
		(self).abs() == 0
	}

	fn absolute(&self) -> Self {
		self.abs()
	}

	fn power(&self, n: f32) -> Self {
		self.pow(n as u32)
	}
}

impl NumberUtils for i64 {
	fn one() -> Self {
		1
	}

	fn approx_zero(&self) -> bool {
		(self).abs() == 0
	}

	fn absolute(&self) -> Self {
		self.abs()
	}

	fn power(&self, n: f32) -> Self {
		self.pow(n as u32)
	}
}

impl NumberUtils for i128 {
	fn one() -> Self {
		1
	}

	fn approx_zero(&self) -> bool {
		(self).abs() == 0
	}

	fn absolute(&self) -> Self {
		self.abs()
	}

	fn power(&self, n: f32) -> Self {
		self.pow(n as u32)
	}
}

impl NumberUtils for isize {
	fn one() -> Self {
		1
	}

	fn approx_zero(&self) -> bool {
		(self).abs() == 0
	}

	fn absolute(&self) -> Self {
		self.abs()
	}

	fn power(&self, n: f32) -> Self {
		self.pow(n as u32)
	}
}

impl NumberUtils for u8 {
	fn one() -> Self {
		1
	}

	fn approx_zero(&self) -> bool {
		(*self) == 0
	}

	fn absolute(&self) -> Self {
		*self
	}

	fn power(&self, n: f32) -> Self {
		self.pow(n as u32)
	}
}

impl NumberUtils for u16 {
	fn one() -> Self {
		1
	}

	fn approx_zero(&self) -> bool {
		(*self) == 0
	}

	fn absolute(&self) -> Self {
		*self
	}

	fn power(&self, n: f32) -> Self {
		self.pow(n as u32)
	}
}

impl NumberUtils for u32 {
	fn one() -> Self {
		1
	}

	fn approx_zero(&self) -> bool {
		(*self) == 0
	}

	fn absolute(&self) -> Self {
		*self
	}

	fn power(&self, n: f32) -> Self {
		self.pow(n as u32)
	}
}

impl NumberUtils for u64 {
	fn one() -> Self {
		1
	}

	fn approx_zero(&self) -> bool {
		(*self) == 0
	}

	fn absolute(&self) -> Self {
		*self
	}

	fn power(&self, n: f32) -> Self {
		self.pow(n as u32)
	}
}

impl NumberUtils for u128 {
	fn one() -> Self {
		1
	}

	fn approx_zero(&self) -> bool {
		(*self) == 0
	}

	fn absolute(&self) -> Self {
		*self
	}

	fn power(&self, n: f32) -> Self {
		self.pow(n as u32)
	}
}

impl NumberUtils for usize {
	fn one() -> Self {
		1
	}

	fn approx_zero(&self) -> bool {
		(*self) == 0
	}

	fn absolute(&self) -> Self {
		*self
	}

	fn power(&self, n: f32) -> Self {
		self.pow(n as u32)
	}
}


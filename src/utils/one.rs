
pub trait DefaultOne {
	fn one() -> Self;
}

impl DefaultOne for f32 {
	fn one() -> Self {
		1.0
	}
}

impl DefaultOne for f64 {
	fn one() -> Self {
		1.0
	}
}

impl DefaultOne for i8 {
	fn one() -> Self {
		1
	}
}

impl DefaultOne for i16 {
	fn one() -> Self {
		1
	}
}

impl DefaultOne for i32 {
	fn one() -> Self {
		1
	}
}

impl DefaultOne for i64 {
	fn one() -> Self {
		1
	}
}

impl DefaultOne for i128 {
	fn one() -> Self {
		1
	}
}

impl DefaultOne for isize {
	fn one() -> Self {
		1
	}
}

impl DefaultOne for u8 {
	fn one() -> Self {
		1
	}
}

impl DefaultOne for u16 {
	fn one() -> Self {
		1
	}
}

impl DefaultOne for u32 {
	fn one() -> Self {
		1
	}
}

impl DefaultOne for u64 {
	fn one() -> Self {
		1
	}
}

impl DefaultOne for u128 {
	fn one() -> Self {
		1
	}
}

impl DefaultOne for usize {
	fn one() -> Self {
		1
	}
}


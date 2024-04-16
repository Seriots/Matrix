#![allow(dead_code)]

use num_traits::ops::mul_add::MulAdd;

use std::ops::Sub;

use crate::fma::Fma;

pub trait Lerp: Sized {
    fn lerp(self, v: Self, t: f32) -> Self;
}

impl<T: Fma + Sub<Output = T> + From<f32> + Clone> Lerp for T {
	fn lerp(self, v: Self, t: f32) -> Self {
        return Fma::fma(v.sub(self.clone()), t.into(), self.clone());
    }
}

pub fn lerp<V: Lerp>(u: V, v: V, t: f32) -> V {
    return u.lerp(v, t);
}

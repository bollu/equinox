use std::ops::{Mul, Add};

pub fn lerp<T>(begin: &T, end: &T, factor: f32) -> T{
	assert!(factor >= 0f32 && factor <= 1f32);
	*begin * (1f32 - factor) + *end * factor
}



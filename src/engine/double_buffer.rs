use std::mem::swap;

//front and back buffer terminology
pub struct DoubleBuffer<T> {
	front: T,
	back: T,
}

impl<T> DoubleBuffer<T> {
	pub fn new(front : T,back : T) -> DoubleBuffer<T> {
		DoubleBuffer { front: front, back: back}
	}

	pub fn front<'a>(&'a mut self) -> &'a mut T {
		&mut self.front
	}

	pub fn back<'a>(&'a self) -> &'a T {
		&self.back
	}

	pub fn flip(&mut self) {
		swap(&mut self.front, &mut self.back);
	}

}


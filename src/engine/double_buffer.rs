//front and back buffer terminology
pub struct DoubleBuffer<T> {
	front: T,
	back: T,
}

impl<T: Clone> DoubleBuffer<T> {
	pub fn new(front: T, back: T) -> DoubleBuffer<T> {
		DoubleBuffer { front: front, back: back }
	}

	pub fn front_and_back<'a>(&'a mut self) -> (&'a T, &'a mut T) {
		let DoubleBuffer{ ref front, ref mut back} = *self;
		(front, back)
	}

	pub fn flip(&mut self) {
		let DoubleBuffer { ref mut front, ref back } = *self;
		front.clone_from(back);
	}
}
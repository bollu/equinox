use engine::math::{vector2, Angle};
use collections::ringbuf::RingBuf;
use collections::deque::Deque;

pub type UniqID = uint;
//yes, this is hard-coded.
pub enum Component {
	Position(vector2),
	Facing(Angle),
}

pub struct Object {
	id: UniqID,
}

pub struct Simulation {
	renderers: RingBuf<&'a rsfml::traits::Drawable:>,
};
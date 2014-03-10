pub type UID = int;

pub struct Position {
	x: f32,
	y: f32,
}

impl Position {
	pub fn new(x: f32, y: f32) -> Position {
		Position { x: x, y: y }
	}
}
//----------------------

pub struct Health {
	maxHP: int,
	currentHP: int,
	invul: bool,
}

impl Health {
	pub fn new(maxHP: int, invul: bool) -> Health {
		Health {maxHP: maxHP, currentHP: maxHP, invul: invul}
	}
}

//-----------------------
/*use Window::Renderer

pub struct Renderer {
	target: Renderer,
	z: int,
	visible: bool,
}

impl Renderer {
	pub fn new(target: RendererType, z: int, visible: bool) -> Renderer {
		Renderer {target: target, z: z, visible: visible}
	}
}
*/
//-----------------------

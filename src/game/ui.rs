use sfml::graphics::{Color, Text, RectangleShape, Font};
use sfml::window::Event;
use engine::event_queue::EventHandler;
use game::colors;
use engine::asd::ASD;
use engine::rendering::RenderQueue;

//UI code corrupts. pretty UI code corrupts absolutely. 

pub fn calculate_text_centering_x<'b>(text: &'b Text, total_x: f32) -> f32{
	total_x * 0.5 - text.local_bounds().width * 0.5
}

pub struct MenuItem<'a, T> {
	text: Text<'a>,
	back: RectangleShape<'a>,
	default_color: Color,
	animator: ASD,

	data: T,
}

pub struct SimpleMenuHandler {
	x: f32,
	y: f32,
	clicked: bool,
	window_closed: bool,
}

impl<'a, T> MenuItem<'a, T> {
	pub fn new(data: T, label: &str, font: &'a Font, font_size: usize, text_color: &Color, back_color: &Color) -> MenuItem<'a, T> {
		
		let mut text = Text::new_init(label, font, font_size).unwrap();
		text.set_color(text_color);

		let mut back = RectangleShape::new().unwrap();
		back.set_fill_color(&colors::invisible);

		MenuItem{data: data,
				text: text, 
				back: back, 
				default_color: *back_color, 
				animator: ASD::new(0f32, 255f32, 0.5, 0.5) 
			}

	}

	pub fn set_bounds(&mut self, total_x: f32, y: f32, item_height: f32) {
		let text_x = calculate_text_centering_x(&self.text, total_x);

		let text_y = y + item_height * 0.5 - self.text.get_local_bounds().height;

		self.text.set_position2f(text_x, text_y);
		
		self.back.set_size2f(total_x, item_height);
		self.back.set_position2f(0 as f32, text_y);
	}

	pub fn update_state(&mut self, dt: f32, mouse_x: f32, mouse_y: f32) {

		self.animator.Tick(dt);
		let mut color = self.default_color;
		color.alpha = self.animator.val() as u8;
		
		self.back.set_fill_color(&color);

		if self.back.get_global_bounds().contains(mouse_x,mouse_y) {
			 self.animator.attack();
		
		} else {
			self.animator.decay();
		}
	}
	
	pub fn contains(&mut self, mouse_x: f32, mouse_y: f32) -> bool{
		self.back.get_global_bounds().contains(mouse_x,mouse_y)
	}


	pub fn push_to_queue(&self, queue: &mut RenderQueue) {
		queue.push(&self.back);
		queue.push(&self.text);
		
	}
}


impl SimpleMenuHandler {
	pub fn new() -> SimpleMenuHandler {
		SimpleMenuHandler { x: 0., y: 0., clicked: false, window_closed: false}
	}

	pub fn reset(&mut self){
		self.clicked = false;
	}

	pub fn should_quit(&self) -> bool {
		self.window_closed
	}
}

impl EventHandler for SimpleMenuHandler {
	fn handle_event(&mut self, event : &Event) {
		match *event {
			Event::MouseMoved { x, y } => {
				self.x = x as f32; 
				self.y = y as f32;
			},

			Event::MouseButtonPressed {..} => {
				self.clicked = true;
			},

			Event::Closed => {
				self.window_closed = true;
			}

			_ => return
		} 
	}	
}

use engine::state::{State, EngineState, NoChange, StateTransition, EngineShutdown};

use engine::resource_loader::{ResourceLoader, Font};
use engine::rendering::{RenderContext, RenderQueue};

use engine::event_queue::{EventQueue, EventHandler};
use engine::asd::{ASD};

use rsfml::graphics::{Color, Text, FloatRect, RectangleShape, Font};
use rsfml::window::event; 

use game::colors;
use game;


pub struct MainMenu<'a>{
	banner: Text<'a>,
	menu_items: ~[MenuItem<'a>],
	handler: MainMenuHandler,
}


enum MenuItemTag {
	TAG_PLAY = 0,
	TAG_OPTIONS = 2,
	TAG_QUIT = 3,
}


struct MenuItem<'a> {
	text: Text<'a>,
	back: RectangleShape<'a>,
	default_color: Color,
	tag: MenuItemTag,

	animator: ASD,
}

struct MainMenuHandler {
	x: f32,
	y: f32,
	clicked: bool,
	window_closed: bool,
}



impl<'a> MenuItem<'a> {
	pub fn new(tag: MenuItemTag, label: &str, font: &'a Font, font_size: uint, text_color: &Color, back_color: &Color) -> MenuItem<'a> {
		
		let mut text = Text::new_init(label, font, font_size).unwrap();
		text.set_color(text_color);

		let mut back = RectangleShape::new().unwrap();
		back.set_fill_color(&colors::invisible);

		MenuItem{ tag: tag,
				text: text, 
				back: back, 
				default_color: *back_color, 
				animator: ASD::new(0f32, 255f32, 0.5, 0.5) 
			}

	}

	pub fn set_bounds(&mut self, total_x: f32, y: f32, item_height: f32) {
		let text_x = MenuItem::calculate_text_x(&self.text, total_x);
		self.text.set_position2f(text_x, y);
		
		self.back.set_size2f(total_x, item_height);
		self.back.set_position2f(0 as f32, y);
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

	pub fn calculate_text_x<'b>(text: &'b Text, total_x: f32) -> f32{
		total_x * 0.5 - text.get_local_bounds().width * 0.5
	}


	fn push_to_queue(&self, queue: &mut RenderQueue) {
		queue.attach(&self.back);
		queue.attach(&self.text);
		
	}
}

impl<'a> MainMenu<'a>{
	pub fn new(loader: &'a ResourceLoader, render_ctx: &RenderContext) -> MainMenu<'a> {

		let top_padding = 10.;
		let banner_menu_padding = 150.;

		let font_size_banner = 90;
		let font_size_menu = 70;
		let font = loader.getFont(~"MenuFont");

		let menu_tags = ~[TAG_PLAY, TAG_OPTIONS, TAG_QUIT];
		let menu_names = ~[&"play", &"options", &"quit"];
		let menu_back_colors =  ~[colors::blue, colors::green, colors::red];
		let mut menu_items = ~[];

		
		let render_dim = render_ctx.viewport_dim;
		let mut current_y = top_padding;
		let mut current_x = 0.;
		
		//banner---------
		let mut banner = Text::new_init("equinox", font, font_size_banner).unwrap();
		banner.set_color(&colors::white);

		current_x = MenuItem::calculate_text_x(&banner, render_dim.x);
		banner.set_position2f(current_x, current_y);
		
		current_y += banner.get_local_bounds().height;
		current_y += banner_menu_padding;
		
		
		//rest of it-----
		let y_spacing = (render_dim.y - current_y) / menu_names.len() as f32;
		
		let mut index = 0;
		for name in menu_names.iter() {
			let menu_tag = menu_tags[index];
			let menu_color = menu_back_colors[index];

			let mut item = MenuItem::new(menu_tag, *name, font, font_size_menu, &colors::white, &menu_color);
			item.set_bounds(render_dim.x, current_y, y_spacing);
			menu_items.push(item);

			current_y += y_spacing;
			index += 1;
			
		}

		return MainMenu { 
			banner: banner, 	
			menu_items: menu_items,
			handler: MainMenuHandler::new() 
		}
	}

	fn Handle_Click(tag: MenuItemTag) -> EngineState{
		match tag {
			TAG_PLAY => StateTransition(game::GAME_STATE_ID as int),
			TAG_OPTIONS => StateTransition(game::OPTIONS_STATE_ID as int),
			Exit => EngineShutdown,
		}
	}
}

impl<'a> State for MainMenu<'a> {
	fn queue_event_handlers(&mut self, event_queue: &mut EventQueue){
		event_queue.attach(&mut self.handler);
	}
	
	fn queue_renderers(&mut self, render_queue: &mut RenderQueue){
		render_queue.set_clear_color(colors::black);
		render_queue.attach(&self.banner);

		for item in self.menu_items.iter() {
			item.push_to_queue(render_queue);
		}
	}

	fn Tick(&mut self, dt: f32) -> EngineState { 
		
		for item in self.menu_items.mut_iter() {
			item.update_state(dt, self.handler.x, self.handler.y);
			
			if self.handler.clicked && item.contains(self.handler.x, self.handler.y) {
				
				return MainMenu::Handle_Click(item.tag);
			}	
		}
		
		self.handler.reset();

		return NoChange;


	}
}

impl MainMenuHandler {
	pub fn new() -> MainMenuHandler {
		MainMenuHandler { x: 0., y: 0., clicked: false, window_closed: false}
	}

	pub fn reset(&mut self){
		self.clicked = false;
	}
}

impl EventHandler for MainMenuHandler {
	fn handle_event(&mut self, event : &event::Event) {
		match *event {
			event::MouseMoved { x, y } => {
				self.x = x as f32; 
				self.y = y as f32;
			},

			event::MouseButtonPressed {button, x, y} => {
				self.clicked = true;
			},

			event::Closed => {
				self.window_closed = true;
			}

			_ => return
		} 
	}	
}
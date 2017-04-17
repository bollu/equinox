//extern crate sfml;

use engine::math::Vector2;
use sfml::graphics::{RenderWindow, Color, Drawable};
use sfml::window::{ContextSettings, VideoMode, Event, WindowStyle};

use std::collections::VecDeque;

pub struct RenderContext {
    viewport_dim: Vector2,
}

pub struct Window {
    window: RenderWindow,
}

impl Window {
    pub fn new(width: usize, height: usize, title: String, fullscreen: bool) -> Window {

        let videoMode = VideoMode::new_init(width, height, 32);
        let style: WindowStyle = if fullscreen {
            WindowStyle::FULLSCREEN
        } else {
            WindowStyle::CLOSE
        };
        let setting = ContextSettings::default();

        Window { window: RenderWindow::new(videoMode, title, style, &setting).unwrap() }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn poll(&mut self) -> Event {
        self.window.poll_event()
    }

    pub fn close(&mut self) {
        self.window.close()
    }

    pub fn clear(&mut self, clear_color: Color) {
        self.window.clear(&clear_color);
    }

    pub fn display(&mut self) {
        self.window.display();
    }

    pub fn width(&self) -> usize {
        self.window.get_size().x as usize
    }

    pub fn height(&self) -> usize {
        self.window.get_size().y as usize
    }

    pub fn get_context(&self) -> RenderContext {
        let size = self.window.get_size();
        RenderContext { viewport_dim: Vector2::new(size.x as f32, size.y as f32) }
    }

    //only use if you're sure
    pub fn get_render_window<'a>(&'a mut self) -> &'a mut RenderWindow {
        &mut self.window
    }
}



pub struct RenderQueue<'a> {
    // There was a colon at the end of Drawable before, it had
    // (probably) something to do with opting out of builtin traits
    // It was also in a couple of other locations too
    renderers: VecDeque<&'a Drawable>,
    clear_color: Color,
}

impl<'a> RenderQueue<'a> {
    pub fn new() -> RenderQueue<'a> {
        RenderQueue {
            renderers: VecDeque::new(),
            clear_color: Color::new_RGB(0, 0, 20),
        }
    }

    pub fn push(&mut self, renderer: &'a Drawable) {
        self.renderers.push_back(renderer);
    }


    pub fn set_clear_color(&mut self, clear_color: Color) {
        self.clear_color = clear_color;
    }

    pub fn render(&mut self, window: &mut Window) {
        window.clear(self.clear_color);

        let render_window = window.get_render_window();

        for renderer in self.renderers.iter() {
            renderer.draw_in_render_window(render_window);
        }

        self.renderers.clear();
    }
}

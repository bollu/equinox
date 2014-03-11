use rsfml::graphics;
use std::hashmap::HashMap;

	
type Key = ~str;


pub enum Resource {
	Font(graphics::Font),
	Color(graphics::Color),
}

pub struct ResourceLoader {
    resources: HashMap<Key, Resource>,

}

impl ResourceLoader {
	pub fn new() -> ResourceLoader {
		ResourceLoader { resources: HashMap::new() }
	}

	pub fn addResource(&mut self, key: Key, resource: Resource) {
		self.resources.insert(key, resource);
	}

	pub fn getFont<'a>(&'a self, key: Key) -> &'a graphics::Font {
		let font = self.getResource(&key);

		match *font {
			Font(ref f) => f,
			_ => fail!("unable to locate font {}", key),
		}
	}

	fn getResource<'a>(&'a self, key: &Key) -> &'a Resource {
		match self.resources.find(key) {
			Some(resource) => return resource,
			None => fail!("unable to find resource {}", *key),
		}
	}
}
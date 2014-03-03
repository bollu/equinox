use std::hashmap::HashMap;
use std::io::stdin;
use std::io::fs::File;
use std::logging;

pub type key = ~str;

pub enum Setting {
	Int(int),
	Float(f32),
	String(~str),
	Vector(f32, f32),
	Bool(bool),
	Error()
}

pub struct Settings {
	keyValues: HashMap<key, Setting>

}

impl Settings {
	fn ParsePair(name: ~str, valueStr: ~str) -> (key, Setting) {

		match name.char_at(0) {

			'i' => {
				let value = from_str::<int>(valueStr).unwrap();
				return (name, Int(value));
			},

			'f' => {
				let value = from_str::<f32>(valueStr).unwrap();
				return (name, Float(value));
			},

			's' => {
				return (name, String(valueStr));
			},

			'v' => {
				let floatPair : ~[&str] = valueStr.split(' ').collect();

				let float1 = from_str::<f32>(floatPair[0]).unwrap();
				let float2 = from_str::<f32>(floatPair[1]).unwrap();
				return (name, Vector(float1, float2));
			},



			'b' => {
				let value = (valueStr == ~"true");
				return (name, Bool(value));
			},

			_ => {
				return (name, Error);
			}
		}	
	}

	fn Parse(data: ~str) -> HashMap<key, Setting> {

		let dataPairs : ~[&str] = data.split('\n').collect();

		for &pair in dataPairs.iter() {
			::debug!("pair: {}", pair);
		}

		let keyValues = HashMap::new();
		keyValues
	}
	
	pub fn new(settingsPath: ~str) -> Settings{
		let path = Path::new(settingsPath);

		if path.exists() {
			let contents = File::open(&path).read_to_str().unwrap();
			let keyValues = Settings::Parse(contents);

			Settings { keyValues: keyValues }
	    }
	    else {
	    	let keyValues = HashMap::new();

	    	Settings { keyValues: keyValues }
	    }


	}


}
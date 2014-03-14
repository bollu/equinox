use std::hashmap::HashMap;
use std::io::fs::File;

pub type key = ~str;


#[deriving(Show)]
pub enum Setting {
	Int(int),
	Float(f32),
	String(~str),
	Vector(f32, f32),
	Bool(bool),
	Error()
}

pub struct Settings {
	key_value_pair: HashMap<key, Setting>

}

impl Settings {
	fn gen_setting(name: &str, valueStr: &str) -> Setting {

		match name.char_at(0) {

			'i' => {
				let value = from_str::<int>(valueStr).unwrap();
				Int(value)
			},

			'f' => {
				let value = from_str::<f32>(valueStr).unwrap();
				Float(value)
			},

			's' => {
				return String(valueStr.to_owned());
			},

			'v' => {
				let floatPair : ~[&str] = valueStr.split(' ').collect();

				let float1 = from_str::<f32>(floatPair[0]).unwrap();
				let float2 = from_str::<f32>(floatPair[1]).unwrap();
				Vector(float1, float2)
			},

			'b' => {
				let value = (valueStr == "true");
				Bool(value)
			},

			_ => {
				Error
			}
		}	
	}

	fn parse(data: ~str) -> HashMap<key, Setting> {

		let mut key_value_pair = HashMap::new();

		let dataLines : ~[&str] = data.split('\n').collect();
		
		for &line in dataLines.iter() {
			let dataPair : ~[&str] = line.split(':').collect();
			
			if dataPair.len() != 2 {
				::warn!("faulty data pair: {}", dataPair);
				continue;
			}

			let name = dataPair[0].trim();
			let valueStr = dataPair[1].trim();

			let setting = Settings::gen_setting(name, valueStr);

			::debug!("name: {}, Setting: {}", name, setting);

			key_value_pair.insert(name.to_owned(), setting);
		}

		::debug!("key_value_pair: {}", key_value_pair);
		key_value_pair
	}
	
	pub fn new(settingsPath: ~str) -> Settings{
		let path = Path::new(settingsPath);

		if path.exists() {
			let contents = File::open(&path).read_to_str().unwrap();
			let key_value_pair = Settings::parse(contents);

			Settings { key_value_pair: key_value_pair }
	    }
	    else {
	    	let key_value_pair = HashMap::new();

	    	Settings { key_value_pair: key_value_pair }
	    }
	}
}
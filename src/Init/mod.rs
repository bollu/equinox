use std::hashmap::HashMap;
use std::io::stdin;
use std::io::fs::File;
use std::logging;

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
	keyValues: HashMap<key, Setting>

}

impl Settings {
	fn genSetting(name: &str, valueStr: &str) -> Setting {

		match name.char_at(0) {

			'i' => {
				let value = from_str::<int>(valueStr).unwrap();
				return Int(value);
			},

			'f' => {
				let value = from_str::<f32>(valueStr).unwrap();
				return Float(value);
			},

			's' => {
				return String(valueStr.to_owned());
			},

			'v' => {
				let floatPair : ~[&str] = valueStr.split(' ').collect();

				let float1 = from_str::<f32>(floatPair[0]).unwrap();
				let float2 = from_str::<f32>(floatPair[1]).unwrap();
				return Vector(float1, float2);
			},

			'b' => {
				let value = (valueStr == ~"true");
				return Bool(value);
			},

			_ => {
				return Error;
			}
		}	
	}

	fn Parse(data: ~str) -> HashMap<key, Setting> {

		let mut keyValues = HashMap::new();

		let dataLines : ~[&str] = data.split('\n').collect();

		for &line in dataLines.iter() {
			let dataPair : ~[&str] = line.split(':').collect();
			
			if(dataPair.len() != 2) {
				::warn!("faulty data pair: {}", dataPair);
				continue;
			}

			let name = dataPair[0].trim();
			let valueStr = dataPair[1].trim();

			let setting = Settings::genSetting(name, valueStr);

			::debug!("name: {}, Setting: {}", name, setting);

			keyValues.insert(name.to_owned(), setting);
		}

		::debug!("keyValues: {}", keyValues);
		return keyValues
	}
	
	pub fn new(settingsPath: ~str) -> Settings{
		let path = Path::new(settingsPath);

		if path.exists() {
			let contents = File::open(&path).read_to_str().unwrap();
			let keyValues = Settings::Parse(contents);

			return Settings { keyValues: keyValues }
	    }
	    else {
	    	let keyValues = HashMap::new();

	    	return Settings { keyValues: keyValues }
	    }


	}


}
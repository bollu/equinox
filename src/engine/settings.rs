use collections::hashmap::HashMap;
use std::io::fs::File;
use std::vec_ng::Vec;

pub type Key = ~str;


#[deriving(Show)]
pub enum Setting {
	Int(int),
	Float(f32),
	String(~str),
	Vector(f32, f32),
	Bool(bool),
	Error()
}

pub struct Settings<'a> {
	priv settings_path: Path,
	priv key_value_pair: HashMap<Key, Setting>

}

impl<'a> Settings<'a> {

	pub fn new(settings_path_raw: &str) -> Settings<'a>{
		let path = Path::new(settings_path_raw);

		if path.exists() {
			let contents = File::open(&path).read_to_str().unwrap();
			let key_value_pair = Settings::parse(contents);

			Settings { settings_path: path, key_value_pair: key_value_pair }
	    }
	    else {
	    	let key_value_pair = HashMap::new();

	    	Settings { settings_path: path, key_value_pair: key_value_pair }
	    }
	}

	pub fn get_setting<'a>(&'a mut self, name: Key, default_value: Setting) -> &'a mut Setting {
		return  self.key_value_pair.find_or_insert(name, default_value)
	}

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
				let floatPair : Vec<&str> = valueStr.split(' ').collect();

				let float1 = from_str::<f32>(*floatPair.get(0)).unwrap();
				let float2 = from_str::<f32>(*floatPair.get(1)).unwrap();
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

	fn parse(data: ~str) -> HashMap<Key, Setting> {

		let mut key_value_pair = HashMap::new();

		let dataLines : Vec<&str> = data.split('\n').collect();
		
		for &line in dataLines.iter() {
			let dataPair : Vec<&str> = line.split(':').collect();
			
			if dataPair.len() != 2 {
			//	warn!("faulty data pair: {}", dataPair);
				continue;
			}

			let name = dataPair.get(0).trim();
			let valueStr = dataPair.get(1).trim();

			let setting = Settings::gen_setting(name, valueStr);

			//debug!("name: {}, Setting: {}", name, setting);
			key_value_pair.insert(name.to_owned(), setting);
		}

		key_value_pair
	}
	
	
}
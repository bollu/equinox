use std::io::fs::File;
use std::io::Writer;
use std::path::Path;
use serialize::{json, Encodable, Decodable};
use std::str::from_utf8;

#[deriving(Encodable, Decodable)]
pub struct Savefile {
	slot: int,
	open: bool,

	level: int,
}


pub fn save_exists(path: &str) -> bool {
	Path::new(path).exists() 
}

pub fn write_save_to_disk(save: &Savefile, path: &str) {
	let mut file = File::create(&Path::new(path));
	{
        let mut encoder = json::Encoder::new(&mut file as &mut Writer);
        save.encode(&mut encoder);
    
	}
}

pub fn read_save_from_disk(path: &str) -> Savefile {
	let mut file = File::open(&Path::new(path));

	let raw_data = file.read_to_end().unwrap();
	let json_str = from_utf8(raw_data).unwrap();
	
	let json_object = json::from_str(json_str).unwrap();
	let mut savefile_decoder = json::Decoder::new(json_object);

	let savefile : Savefile = Decodable::decode(&mut savefile_decoder);

	savefile
}

impl Savefile {
	pub fn new(slot: int, open: bool, level: int) -> Savefile {
		Savefile {
			slot: slot,
			open: open,
			level: level,
		}
	}
}
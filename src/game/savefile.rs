use std::fs::File;
use std::io::Write;
use std::path::Path;
use rustc_serialize::{json, Encodable, Decodable};
use std::str::from_utf8;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Savefile {
	slot: isize,
	open: bool,

	level: isize,
}


pub fn save_exists(path: &str) -> bool {
	Path::new(path).exists() 
}

pub fn write_save_to_disk(save: &Savefile, path: &str) {
	let mut file = File::create(&Path::new(path));
	{
        let mut encoder = json::Encoder::new(&mut file as &mut Write);
        save.encode(&mut encoder);
    
	}
}

pub fn read_save_from_disk(path: &str) -> Savefile {
	let mut file = File::open(&Path::new(path));

	let raw_data = file.read_to_end().unwrap();
	let json_str = from_utf8(raw_data).unwrap();
	
	let json_object = json::decode(json_str).unwrap();
	let mut savefile_decoder = json::Decoder::new(json_object);

	let savefile : Savefile = Decodable::decode(&mut savefile_decoder);

	savefile
}

impl Savefile {
	pub fn new(slot: isize, open: bool, level: isize) -> Savefile {
		Savefile {
			slot: slot,
			open: open,
			level: level,
		}
	}
}

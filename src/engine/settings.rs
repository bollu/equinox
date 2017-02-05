use std::collections::HashMap;
use std::fs::File;
use std::vec::Vec;
use std::path::Path;
use std::string;
use self::Setting::*;

pub type Key = string::String;


#[derive(Debug)]
pub enum Setting {
    Int(isize),
    Float(f32),
    String(Key),
    Vector(f32, f32),
    Bool(bool),
    Error,
}

pub struct Settings {
    key_value_pair: HashMap<Key, Setting>,
    settings_path: Path,
}

impl Settings {
    pub fn new(settings_path_raw: &str) -> Settings {
        let path = Path::new(settings_path_raw);

        if path.exists() {
            let contents = File::open(&path).read_to_str().unwrap();
            let key_value_pair = Settings::parse(contents);

            Settings {
                settings_path: path,
                key_value_pair: key_value_pair,
            }
        } else {
            let key_value_pair = HashMap::new();

            Settings {
                settings_path: path,
                key_value_pair: key_value_pair,
            }
        }
    }

    pub fn get_setting(&mut self, name: Key, default_value: Setting) -> &mut Setting {
        return self.key_value_pair.find_or_insert(name, default_value);
    }

    fn gen_setting(name: &str, valueStr: &str) -> Setting {

        match name.char_at(0) {

            'i' => {
                let value = isize::from_str(valueStr).unwrap();
                Int(value)
            }

            'f' => {
                let value = f32::from_str(valueStr).unwrap();
                Float(value)
            }

            's' => {
                return String(valueStr.to_owned());
            }

            'v' => {
                let floatPair: Vec<&str> = valueStr.split(' ').collect();

                let float1 = f32::from_str(*floatPair.get(0)).unwrap();
                let float2 = f32::from_str(*floatPair.get(1)).unwrap();
                Vector(float1, float2)
            }

            'b' => {
                let value = valueStr == "true";
                Bool(value)
            }

            _ => Error,
        }
    }

    fn parse(data: Key) -> HashMap<Key, Setting> {

        let mut key_value_pair = HashMap::new();

        let dataLines: Vec<&str> = data.split('\n').collect();

        for &line in dataLines.iter() {
            let dataPair: Vec<&str> = line.split(':').collect();

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

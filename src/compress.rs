use std::fs;
use base64::{Engine as _, engine::general_purpose};


pub fn generate_dir_assets(dir: &str) -> String {
    let mut val: &str;
    let mut val_string: String;

    let paths = fs::read_dir(dir).unwrap();

    let mut result: String = Default::default();
    for _path in paths {
        let path = _path.unwrap();
        let contents = fs::read_to_string(path.path()).unwrap();
        
        let whyyyyy = general_purpose::STANDARD.encode(contents.as_bytes());
        val = whyyyyy.as_str();
        val_string = " \"".to_string() + val + &"\"".to_string();

        result += format!("assets.insert(\"{}\".to_string(), {}.to_string());", path.path().to_str().unwrap() /* i hate this */, val_string).as_str();
    }
    result
}


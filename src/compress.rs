use std::{fs, io::{BufReader, Read, Write}};
use base64::{Engine as _, engine::general_purpose};


fn generate_dir_assets(dir: &str) -> String {
    let mut val: &str;
    let mut val_string: String;

    let paths = fs::read_dir(dir).unwrap();

    let mut result: String = Default::default();
    for _path in paths {
        let path = _path.unwrap();
        
        let file = fs::File::open(path.path().to_str().unwrap()).expect("Failed to open file");
        let mut reader = BufReader::new(&file);
        let mut contents = Vec::new();
        reader.read_to_end(&mut contents).unwrap();
        
        let whyyyyy = general_purpose::STANDARD.encode(contents);
        val = whyyyyy.as_str();
        val_string = " \"".to_string() + val + &"\"".to_string();

        result += format!("map.insert(\"{}\".to_string(), {}.to_string());", path.path().to_str().unwrap().replace("\\", "/") /* i hate this */, val_string).as_str();
    }
    result
}


pub fn create_assets_file(dirs: Vec<&str>, output_file: &str) {
    let mut data: String = "use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose};\n\n
lazy_static::lazy_static! {
    static ref ASSETS: HashMap<String, String> = {
        let mut map: HashMap<String, String> = HashMap::new();
".to_string(); 
    for dir in dirs {
        data += &("\t\t".to_string() + &generate_dir_assets(&dir));
    }
    data += &"
        map
    };
}\n\n
pub fn get_asset(name: &str) -> String {
    let b64_contents = ASSETS.get(name).unwrap();
    let contents = general_purpose::STANDARD.decode(b64_contents).unwrap();
    String::from_utf8_lossy(&contents).to_string()
}".to_string();

    println!("{}", data);

    let mut file = fs::File::create(&(output_file.to_string() + &".rs")).expect(format!("Failed to create output file '{}'", output_file).as_str());
    file.write_all(data.as_bytes()).unwrap();
}

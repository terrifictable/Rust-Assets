use std::process::{Command, Stdio};


lazy_static::lazy_static! {
    static ref BUILD_BRANCH_NAME: String = run_command("git", vec!["rev-parse", "--abbrev-ref", "HEAD"]).replace("\n", "");
    static ref BUILD_COMMIT: String = run_command("git",  vec!["rev-parse", "--short", "HEAD"]).replace("\n", "");

    pub static ref VERSION_STRING: String = get_version_string();   
}


// pub(crate) fn write_help() {
//     println!("rust-assets {}
//     -h | --help     : show help
//     -v | --version  : show version
//     -o | --output   : output filename
//     -d | --dir      : dir to include (recursiveley)\n", VERSION_STRING.as_str());
// }

pub(crate) fn write_version() {
    println!("rust-assets v{}", VERSION_STRING.as_str())
}




fn get_version_string() -> String {
    format!("0.1.0 ({}/{})", BUILD_BRANCH_NAME.as_str(), BUILD_COMMIT.as_str())
}

fn run_command(command: &str, args: Vec<&str>) -> String {
    let output = Command::new(command)
            .args(args)
            .stdout(Stdio::piped())
            .output()
            .unwrap_or_else(|e| {
                panic!("Failed to execute process: {}", e)
            });
    
    String::from_utf8(output.stdout).unwrap()
}


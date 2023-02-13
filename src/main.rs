use std::process::exit;
use clap::Parser;

mod cli_utils;
mod compress;

#[derive(Parser,Debug)]
struct ArgsParser {
    #[clap(short, long, action)]
    version: bool,
    
    #[clap(short, long, default_value_t = String::from("assets.rs"))]
    output: String,
    
    #[clap(short, long, default_value_t = String::from(""))]
    dir: String,
}

fn main() {
    let args = ArgsParser::parse();

    if args.version {
        cli_utils::write_version();
        exit(0);
    }

    if !args.output.is_empty() && !args.dir.is_empty() {
        compress::create_assets_file(vec![args.dir.as_str()], args.output.as_str());
    }
}

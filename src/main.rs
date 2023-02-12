#[allow(unused)]

mod compress;

fn main() {

    let data = compress::generate_dir_assets("test");
    println!("{}", data);

}

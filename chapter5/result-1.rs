use std::io::Read;
use std::path::Path;
use std::fs::File;

fn main() {
    let path = Path::new("data.txt");
    // let file = File::open(&path);
    // let mut s = String::new();

    // file.read_to_string(&mut s);
    // println!("Read the String: {}", s);

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error while opening the file {:?}", err);
            panic!();
        }
    };
    println!("file is {:?}", file);
}
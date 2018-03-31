use std::io::Read;
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("data.txt");
    let mut file = File::open(&path).expect("Error while opening file data.txt");
    let mut s = String::new();
    file.read_to_string(& mut s).expect("error while reading the contents");
    println!("read the string {:?}", s);
}
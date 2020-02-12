use std::fs::File;
use std::io::Read;

fn main() {
    println!("Ready to panic?");

    let mut f = File::open("hello.txt").expect("Unable to open file.");

    let mut s = String::new();
    f.read_to_string(&mut s);
    println!("{}", s);
}

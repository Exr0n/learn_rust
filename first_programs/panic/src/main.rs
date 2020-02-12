use std::fs::File;
use std::io::Read;

fn main() {
    println!("Ready to panic?");

    let mut f = File::open("hello.txt").expect("Unable to open file.");

    let mut s = String::new();
    f.read_to_string(&mut s);
    
    println!("{}", s);
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
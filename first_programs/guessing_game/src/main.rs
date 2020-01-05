use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Number Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    println!("Please input your guess:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess) // input
        .expect("Failed to read line!"); // throws string if enum returned by read_line has type Error

    let guess: u32 = guess.trim().parse() // shadowing is legal
        .expect("Failed to parse number!"); // trim or parse failed
    
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) { // switch statement to compare
        Ordering::Less => println!("That was too small!"),
        Ordering::Greater => println!("That was too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

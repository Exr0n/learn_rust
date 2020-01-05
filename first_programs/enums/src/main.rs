
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter{ year: u32 },
}
impl Coin {
    fn value(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter{year} => {
                println!("Quarter from {}!", year);
                25
            }
        }
    }
}

fn main() {

    let my_coin = Coin::Dime;
    let my_other_coin = Coin::Quarter{year: 2006};

    println!("Value 1: {}", my_coin.value());
    println!("Value 2: {}", my_other_coin.value());
}

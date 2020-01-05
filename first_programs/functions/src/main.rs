fn main() {
    println!("Hello, world!");
    
    another(3);
    println!("Sum of 5 and 6 is {}!", add(5, 6));

    let mut i = 0;
    println!("Loop returned {}!", loop {
        i += 1;
        if i >= 10
        {
            break i;
        }
    });

    println!("Recurse came back with {}!", plus_one(if true {1} else if true {3} else {2}));

    for n in 1..20 {
        println!("This is a for loop range: {}!", n);
    }
}

fn another(num: i32) {
    println!("Yay! The num is: {}", num);
}

fn add(lhs: i32, rhs: i32) -> i32 {
    lhs+rhs
}

fn plus_one(num: i32) -> i32 {
    if num < 20 { plus_one(num+1) } else { 20 }
}
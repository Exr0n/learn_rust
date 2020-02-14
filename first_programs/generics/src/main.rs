use std::cmp::PartialOrd;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'e', 'l', 'l', 'o', 'w'];

    let largest_number = largest(&number_list);

    println!("The largest number is {}", largest_number);
    println!("The largest char is {}", largest(&char_list));
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for number in list {
        if *number > *largest {
            largest = number;
        }
    }
    largest
}
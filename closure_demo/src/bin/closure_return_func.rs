use std::{num, result};

fn get_closure() -> impl Fn(i32) -> i32 {
    |number| number * number * number
}

fn main() {
    let cubed = get_closure();
    let result = cubed(5);
    println!("{}", result)
}

use std::vec::Vec;

fn output_sequence(numbers: &Vec<i32>) {
    for n in numbers.iter() {
        println!("{}", n)
    }
}

fn generate_sequence(limit: u8) -> Vec<u8> {
    (1..=limit).collect()
}

fn main() {
    println!("Hello, world!");
    let numbers = vec![1, 2, 3, 4, 5];

    // for n in numbers {
    //     println!("{}", n)
    // }

    output_sequence(&numbers);
    output_sequence(&numbers);

    println!("{:?}", generate_sequence(10))
}

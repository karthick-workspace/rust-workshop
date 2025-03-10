fn main() {
    let hello = || println!("Hello, world!");
    hello();

    let cubed = |number: usize| -> usize { number * number * number };

    let value = 5;
    let result = cubed(value);
    println!("result={}", result);

    let mut values1 = (5, 10);
    let swap_values = || (values1.1, values1.0);
    // let values2 = &mut values1;
    let result = swap_values();

    println!("{:?}", result);

    let a = 1;
    let b = 2;

    let adder = |prefix: String| println!("{} {}", prefix, a + b);
    adder("Add Operation:".to_string())
}

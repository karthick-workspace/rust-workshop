fn main() {
    let mut values1 = (5, 10);

    let swap_values = || (values1.1, values1.0);
    let mut result = swap_values();
    result = swap_values();
    println!("{:?}", result);

    let values = &mut values1;
}
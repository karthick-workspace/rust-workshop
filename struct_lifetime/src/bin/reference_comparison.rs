use std::ptr;

fn main() {
    let num_of_eggs = 10;
    let num_of_pizza = 10;

    let eggs = &num_of_eggs;
    let pizza = &num_of_pizza;

    let result = ptr::eq(eggs, pizza);

    println!("Equal={:?}", result);
    println!("{}", eggs == pizza)
}
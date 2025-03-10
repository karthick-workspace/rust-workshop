fn do_something<T: Default>() -> T {
    let value: T = T::default();
    value
}

fn main() {
    let result = do_something::<i8>();
    println!("{:?}", result)
}

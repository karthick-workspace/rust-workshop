fn swap<T, U>(tuple: (T, U)) -> (U, T) {
    (tuple.1, tuple.0)
}

fn main() {
    let tuple1 = (10, "ten");
    let result = swap(tuple1);
    println!("{:?}", result);
}

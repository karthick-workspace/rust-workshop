fn do_something<'a>(ref1: &'a i32) -> &'a i32 {
    return ref1;
}

fn main() {
    let num1 = 1;
    let result = do_something(&num1);
    println!("{}", result);
    println!("{}", num1);
}

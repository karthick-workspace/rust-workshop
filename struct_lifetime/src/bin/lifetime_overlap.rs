fn do_something<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    x
}

fn main() {
    let result;
    let num1 = 1;
    let num2 = 2;
    {
        result = do_something(&num1, &num2);
    }
    println!("{}", result);
}
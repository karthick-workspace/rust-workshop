fn do_something(ref1: &i32) -> &i32 {
    return ref1;
}

fn main() {
    // lifetime of 'a
    let ref1 = 1;
    {
        // lifetime of 'b
        let result;
        result = do_something(&ref1);
        println!("{}", result)
    }
    println!("{}", ref1);
}

struct Data<'a> {
    field1: &'a i32,
}

impl<'a> Data<'a> {
    fn do_something<'b>(self: &Self, ref1: &'b i32) -> &'b i32 {
        ref1
    }

    fn do_something2<'b>(self: &Self, ref1: &'a i32, ref2: &'b i32) -> &'a i32 {
        ref1
    }
}
fn main() {
    let num1 = 1;
    let obj = Data { field1: &num1 };
    let result;
    {
        let num2 = 2;
        result = obj.do_something(&num2);
    }
    println!("{}", result)
}
trait Atrait {
    fn get_name(&self) {
        println!("A-trait");
    }
}

trait Btrait {
    fn get_name(&self) {
        println!("B-trait");
    }
}

struct MyStruct {}

impl Atrait for MyStruct {}
impl Btrait for MyStruct {}

fn main() {
    let obj = MyStruct {};
    Atrait::get_name(&obj);
}

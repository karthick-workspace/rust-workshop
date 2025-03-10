trait ATrait {
    fn display_name();
}

struct MyStruct {}

impl ATrait for MyStruct {
    fn display_name() {
        println!("ITrait::MyStruct");
    }
}

impl MyStruct {
    fn display_name() {
        println!("MyStruct");
    }
}

fn main() {
    let obj = MyStruct {};

    MyStruct::display_name();

    <MyStruct as ATrait>::display_name();
}

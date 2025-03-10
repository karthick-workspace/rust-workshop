use std::fmt::Error;
use std::io::Write;

fn main() {
    // Immutable variable
    let x = 5;
    println!("v is {}", x);

    println!("Hello, world!");

    // Mutable variable
    let mut y = 10;
    println!("y is {}", y);

    y = 20;

    println!("y is {}", y);

    const SECONDS: i8 = 60;
    println!("SECONDS:{}", SECONDS);

    println!("{y:?}");
    println!("{y}");

    let s1 = format!("{} {} {}", 1, 2, 3);
    println!("{}", s1);

    let s2 = format!("{0} {0}", 10);
    println!("{0}", s2);

    let s3 = format!("{a}", a = 10);
    println!("{s3}");

    write_macro_demo();

    dbg_macro_demo();

    //todo_demo_macro();

    unimplemented_macro_demo();
}

fn write_macro_demo() -> Result<(), Error> {
    let mut buf = Vec::new();

    write!(&mut buf, "Hello {} times", 42);
    let text = String::from_utf8_lossy(&buf);

    println!("{text}");

    Ok(())
}

fn dbg_macro_demo() {
    let a = 963;
    let x = dbg!(a + 36);
    assert_eq!(x, 999);

    let b = String::from("Rust");
    let y = dbg!(&b);
    println!("{y}")
}

fn todo_demo_macro() -> Option<bool> {
    todo!("todo or not to do")
}

fn unimplemented_macro_demo() {
    unimplemented!("stub")
}
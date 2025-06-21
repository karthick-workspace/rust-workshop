use std::io::{self, Write};

fn accept() -> bool {
    print!("Do you want to continue (y or n)? ");
    io::stdout().flush().unwrap();

    let mut answer = String::new();
    io::stdin().read_line(&mut answer);

    if answer.trim().eq_ignore_ascii_case("Y") {
        return true;
    }
    return false;
}

fn main() {
    if accept() {
        println!("Hello, world!");

        let i1: i32 = 7.2 as i32; // i1 becomes 7

        let i3: i32 = { 7.2 as i32 };

        println!("{}", i1);

        println!("{}", i3);

        let b: [char; 6] = ['0', '1', '2', '3', '4', '5'];
        let a: *const char = &b[3];

        // Derefence a to get the object it point to
        let x: char = unsafe { *a };

        println!("The character at index 3 is {}", x)
    } else {
        println!("Operation cancelled!")
    }
}

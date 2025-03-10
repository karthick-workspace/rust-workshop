use std::fmt::{format, Display};

fn border_value<T: Display>(value: T) -> String {
    let formatted = format!("* {} *", value);
    let len = formatted.len();
    let line = "*".repeat(len).to_string();
    format!("{}\n{}\n{}", line, formatted, line)
}

fn border_value2<T>(value: T) -> String
where
    T: Display,
{
    let formatted = format!("* {} *", value);
    let len = formatted.len();
    let line = "*".repeat(len).to_string();
    format!("{}\n{}\n{}", line, formatted, line)
}

fn largest<T: Display + Ord>(arg1: T, arg2: T) {
    match arg1.cmp(&arg2) {
        std::cmp::Ordering::Less => println!("{} < {}", arg1, arg2),
        std::cmp::Ordering::Equal => println!("{} < {}", arg1, arg2),
        std::cmp::Ordering::Greater => println!("{} > {}", arg1, arg2),
    }
}

fn largest2<T>(arg1: T, arg2: T)
where
    T: Display + Ord,
{
    match arg1.cmp(&arg2) {
        std::cmp::Ordering::Less => println!("{} < {}", arg1, arg2),
        std::cmp::Ordering::Equal => println!("{} < {}", arg1, arg2),
        std::cmp::Ordering::Greater => println!("{} > {}", arg1, arg2),
    }
}

fn do_something<T>(arg1: T)
where
    T: Display,
{
    println!("{}", arg1)
}

fn main() {
    println!("{}", border_value('a'));

    largest(10, 20);

    do_something("Hello Rust");
    println!("{}", border_value2('2'));
    largest2("ten".to_string(), "twenty".to_string());
}

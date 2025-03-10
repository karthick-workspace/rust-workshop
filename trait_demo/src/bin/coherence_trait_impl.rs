use std::fmt::Debug;

trait Printable {
    fn print(&self)
    where
        Self: Debug,
    {
        println!("{:?}", self);
    }
}

impl<T> Printable for T {}

fn main() {
    let number = 12;
    number.print();
}

/*
The trait system in Rust has been carefully constructed to ensure clear and unambiguous trait implementations, thus avoiding any potential conflicts that may arise when multiple crates implement the same trait for the same type.
This fundamental idea, referred to as trait coherence, holds immense significance in upholding code accuracy and preventing unintended clashes between different implementations of a shared trait.

Imagine a scenario where you aim to offer a common way to print types from external crates. By defining a trait that provides a printing method and implementing it for types from external crates,
you can ensure that your code remains free from conflicts. Trait coherence ensures that there won’t be multiple conflicting implementations of the same trait for a given type, promoting code stability and predictability.
*/

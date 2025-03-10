trait Negate {
    fn negate(&self) -> Self;
}
impl<T: Clone + std::ops::Neg<Output = T>> Negate for T {
    fn negate(&self) -> Self {
        -self.clone()
    }
}

fn main() {
    let number = 12;
    println!("Negated:{}", number.negate())
}

/*
In Rust, blanket implementations hold significant power as they allow you to effortlessly apply a trait to all types that meet specific criteria.
This approach proves highly advantageous when you seek to establish a common functionality for a wide range of types, eliminating the need for implementing the trait individually for each type.

To exemplify this, let’s consider the creation of a trait called Negate that aims to reverse the sign of a variable value. Here’s an example that effectively demonstrates this concept:

With this blanket implementation, you can now use the negate method on various numeric types without having to define it separately for each one. This approach promotes code reusability and ensures consistent behavior across a wide range of types.

*/

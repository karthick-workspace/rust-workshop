use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
struct Wrapper<T> {
    internal: T,
}

impl<T: Copy + Display> Wrapper<T> {
    fn get(&self) -> T {
        self.internal
    }

    fn set(&mut self, data: T) {
        self.internal = data
    }

    fn display<U: Display>(&self, prefix: U, suffix: U) {
        println!("{} {} {}", prefix, self.internal, suffix)
    }

    fn perform<F>(mut self, operation: F) -> Self
    where
        Self: Copy + Clone,
        F: Fn(T) -> T,
    {
        self.internal = operation(self.internal);
        self
    }
}

fn main() {
    let obj1 = Wrapper { internal: 2 };
    println!("{}", obj1.get());

    let obj2 = Wrapper { internal: 1.1 };
    println!("{}", obj2.get());

    obj2.display("****", "####");

    let objCopy = obj1.perform(|arg1| arg1 * arg1);
    println!("{:?} {:?}", obj1, objCopy)
}

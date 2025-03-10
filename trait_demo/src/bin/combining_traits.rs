trait Drawable {
    fn draw(&self);
}

trait Printable {
    fn print(&self);
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl<T> Drawable for T
where
    T: Printable + std::fmt::Debug,
{
    fn draw(&self) {
        println!("Drawing: {:?}", self)
    }
}

impl Printable for Complex {
    fn print(&self) {
        println!("Printing: {} + {}i", self.real, self.imaginary)
    }
}

fn main() {
    let complex = Complex {
        real: 1.0,
        imaginary: 2.0,
    };
    complex.draw();
    complex.print();
}

trait Shape {
    fn draw(&self) {
        println!("Drawing....")
    }
    fn erase(&self);
}

struct Rectangle {}
struct Ellipse {}

impl Shape for Rectangle {
    fn erase(&self) {
        println!("Erased Rectangle")
    }
}

impl Shape for Ellipse {
    fn draw(&self) {
        println!("Ellipse Drawing")
    }
    fn erase(&self) {
        println!("Ellipse erased")
    }
}

fn main() {
    let rect = Rectangle {};
    rect.draw();

    let ellipse = Ellipse {};
    ellipse.draw();
    ellipse.erase();
}

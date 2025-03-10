use crate::geometry::Rectangle;

mod geometry {
    pub struct Rectangle{
        pub width:u32,
        pub height:u32,
    }

    impl  Rectangle {
        pub fn area(&self) -> u32 {
            self.width * self.height
        }
    }
}

mod geo {
    pub struct Rectangle {
        width:u32,
        height:u32,
    }

    impl  Rectangle {
        pub fn area(&self) -> u32 {
            self.width * self.height
        }


    }

    pub fn new(w:u32,h:u32) -> Rectangle{
        Rectangle{ width: w, height: h }
    }


}

fn main() {
    println!("Hello, world!");

    let rect:Rectangle = geometry::Rectangle{height:10,width:20,};

    println!("Rectangle area = {}.",rect.area());

    let rect = geo::new(10,20);

    println!("Rectangle area = {}.",rect.area());

}


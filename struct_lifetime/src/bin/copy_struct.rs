#[derive(Debug, Clone, Copy)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGB {
    fn new() -> Box<RGB> {
        Box::new(RGB {
            red: 50,
            green: 50,
            blue: 50,
        })
    }
    fn is_gray(self: &Self) -> bool {
        (self.red == self.blue) && (self.blue == self.green)
    }

    fn invert(&mut self) {
        self.red = 255 - self.red;
        self.green = 255 - self.green;
        self.blue = 255 - self.blue;
    }
}

fn main() {
    let dark = RGB::new();

    let mut light_dark = dark;

    light_dark.blue += 125;
    light_dark.green += 125;
    light_dark.red += 125;

    dbg!(dark);
    dbg!(light_dark);

    light_dark.invert();
    println!("{}", light_dark.is_gray());

    dbg!(light_dark);
}

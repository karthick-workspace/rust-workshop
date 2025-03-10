use std::ops;

#[derive(Debug, Clone, Copy)]
struct RGB {
    red: u8,
    blue: u8,
    green: u8,
}
impl ops::Add for RGB {
    type Output = RGB;

    fn add(self, rhs: RGB) -> Self::Output {
        RGB {
            red: self.red + rhs.red,
            blue: self.blue + rhs.blue,
            green: self.green + rhs.green,
        }
    }
}

impl ops::Add<(u8, u8, u8)> for RGB {
    type Output = RGB;

    fn add(self, rhs: (u8, u8, u8)) -> Self::Output {
        RGB {
            red: self.red + rhs.0,
            blue: self.blue + rhs.1,
            green: self.green + rhs.2,
        }
    }
}

fn main() {
    let color1 = RGB {
        red: 200,
        blue: 75,
        green: 125,
    };
    let color2 = RGB {
        red: 50,
        blue: 75,
        green: 25,
    };

    let color3 = color1 + color2;
    println!("{:?}", color3);
}

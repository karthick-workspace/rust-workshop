#[derive(Debug)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let dark = RGB {
        red: 50,
        green: 50,
        blue: 50,
    };
    let orange = RGB {
        red: 255,
        green: 165,
        blue: 0,
    };

    dbg!(&dark);
    dbg!(&orange);
}

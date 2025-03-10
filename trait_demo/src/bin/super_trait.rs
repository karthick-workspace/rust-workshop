struct Text {}

// trait
trait Font {
    fn set_font(&mut self, font_name: String);
}

trait Unicode {}

trait FontWithStyle: Font + Unicode {
    fn set_style(&mut self, bold: bool, italics: bool);
}

// trait implementations
impl Font for Text {
    fn set_font(&mut self, font_name: String) {
        println!("applying font...")
    }
}

impl FontWithStyle for Text {
    fn set_style(&mut self, bold: bool, italics: bool) {
        println!("applying style...")
    }
}

impl Unicode for Text {}

fn main() {
    let mut text = Text {};
    text.set_font("Arial".to_owned());
    text.set_style(true, false);
}

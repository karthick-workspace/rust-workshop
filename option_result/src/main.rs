fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(format!("Divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn show_divide(num: f64, den: f64) {
    match divide(num, den) {
        Ok(val) => println!("{} / {} = {}", num, den, val),
        Err(erg_msg) => println!("Cannot divide {} by {} : {}", num, den, erg_msg)
    }
}

fn main() {
    let mut v = vec![11, 22, 33];

    for _ in 0..5 {
        let item = v.pop();
        match item {
            Some(number) => println!("{}", number),
            None => println!("#")
        }
    }

    println!("{:?}, {:?}", divide(8., 2.), divide(8., 0.));

    show_divide(8., 2.);
    show_divide(8., 0.);
}

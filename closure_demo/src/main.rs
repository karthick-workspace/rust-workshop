pub fn make_quadratic(a: f64, b: f64, c: f64) -> impl Fn(f64) -> f64 {
    move |x| a * x * x + b * x + c
}

fn main() {
    let x = 10.0;
    let quad_fn = make_quadratic(5.0, 4.0, 3.0);
    println!("{}", quad_fn(x));
}

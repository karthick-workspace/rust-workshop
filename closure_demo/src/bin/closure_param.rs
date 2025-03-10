enum Calculation {
    Cubed,
    Quad,
}

fn get_result(run: impl Fn(i32) -> i32, value: i32) -> i32 {
    run(value)
}
fn main() {
    let cubed = |value: i32| value * value * value;
    let quad = |value: i32| value * value * value * value;

    let calculation = Calculation::Cubed;

    let result = match calculation {
        Calculation::Cubed => get_result(cubed, 5),
        Calculation::Quad => get_result(quad, 5),
    };

    println!("{}", result)
}

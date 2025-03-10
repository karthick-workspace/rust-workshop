fn main() {
    if let Some(arg) = std::env::args().nth(0) {
        println!("{}", arg)
    }

    //iterate and display arg
    for arg in std::env::args() {
        println!("{}", arg)
    }

    // collect all the arg and display
    let all_arg = std::env::args().collect();
    println!("{}", all_arg);
}

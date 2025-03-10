fn do_closure(run: impl Fn()) {
    run()
}

fn main() {
    let display = || println!("message");
    do_closure(display);
}

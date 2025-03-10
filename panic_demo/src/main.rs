use std::any::Any;
use std::panic;
fn get_data(request: usize) -> Result<i8, Box<dyn Any + Send>> {
    let vec: Vec<i8> = (0..100).collect();


    let result = panic::catch_unwind(|| {
        vec[request]
    });
    result
}

fn unwrap_panic() {
    let items = vec![1, 2, 3];
    let value = items.get(5);
    println!("{:?}", value.unwrap());
}
fn main() {
    panic::set_hook(Box::new(|_| {
        println!("Panic error handled")
    }));
    let data: usize = 105;
    let result = get_data(data);
    match result {
        Ok(value) => println!("{:?}", value),
        Err(_) => println!("not ok")
    }

    unwrap_panic();
}

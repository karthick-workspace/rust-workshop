fn vec_push_within<T>(vec1: &mut Vec<T>, value: T) -> Result<usize, Vec<T>> {
    let capacity = vec1.capacity();
    let len = vec1.len();

    let diff = capacity - len;

    if diff != 0 {
        vec1.push(value);
        Ok(diff - 1)
    } else {
        Err(vec![value])
    }
}

fn main() {
    let mut vec1 = Vec::with_capacity(2);
    vec1.push(1);
    vec1.push(2);
    let result = vec_push_within(&mut vec1, 3);

    match result {
        Ok(_) => println!("Original {:?}", vec1),
        Err(value) => println!("New {:?}", value),
    }
}

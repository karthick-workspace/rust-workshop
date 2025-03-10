use std::collections::HashMap;

fn main() {
    let array_1 = [1, 2, 3, 4];
    array_1.into_iter().for_each(move |f| println!("{}", f));

    for element in array_1.into_iter().enumerate() {
        println!("{:?}", element.1);
    }

    let mut vec1 = Vec::new();
    vec1.push(1);

    println!("{:?}", vec1);

    let vec2 = vec![1, 2, 3, 4];

    println!("{:?}", vec2);

    let mut map = HashMap::new();
    map.insert("K1", "V1");

    println!("{:?}", map);

    match map.get(&"K1") {
        Some(value) => println!("Found {}", value),
        None => println!("Not Found"),
    }

    for (key, value) in map {
        println!("{} {}", key, value)
    }
}

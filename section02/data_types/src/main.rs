fn main() {
    let x: i8 = 10;
    println!("x is {}", x);

    let y: u8 = 10;
    println!("{}", y);

    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);

    let byte = b'A';

    println!("{}", byte);

    let x1 = 2.0; // float variable
    let y1: f32 = 1.0;

    let t = true;
    let f: bool = false;

    let c = 'c';
    println!("{}", c);

    let tup = (400, "hi", true);
    println!("{}", tup.0);

    let (t1, t2, t3) = tup;
    println!("{},{},{}", t1, t2, t3);

    let array = [1, 2, 3];
    println!("{}", array[0]);

    let mut array2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", array2[0]);

    array2[0] = 100;

    println!("{}", array2[0]);

    println!("{:?}", array2);

    let mut nums = vec![1, 2, 3];

    nums.push(4);

    println!("{:?}", nums);

    nums.pop();

    println!("{:?}", nums);

    let mut string_vec = Vec::new();
    string_vec.push("Test");
    string_vec.push("string");


    println!("{:?}", string_vec);
    string_vec.reverse();
    println!("{:?}", string_vec);

    let mut vec_cap = Vec::<i32>::with_capacity(2);
    println!("{}", vec_cap.capacity());

    let vec_range: Vec<i32> = (0..5).collect();
    println!("{:?}", vec_range);

    let slice_vec: &[i32] = &vec_range;
    println!("{:?}", slice_vec);

    let slice_vec1: &[i32] = &vec_range[2..4];
    println!("{:?}", slice_vec1);

    let name = String::from("Karthick");
    let course = "Rust".to_string();
    let new_name = name.replace("k", "K");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    let str1 = "\x52\x75\x73\x74";
    println!("{}", str1);

    print_pharse("print this argument");


    let mut index = 0;
    'counter: loop {
        println!("Count {}", index);
        let mut decrease = 5;
        loop {
            println!("Decreasing:{}",decrease);
            if decrease == 4 {
                break;
            }
            if index == 2 {
                break 'counter;
            }
            decrease -= 1;
        }
        index += 1;
    }

    let mut num = 0;
    while num < 5 {
        println!("Num {}",num);
        num += 1;
    }

    let vec:Vec<i8> = (0..10).collect();

    for element in vec{
        println!("{}",element);
    }

    for number in (0..4).rev(){
        println!("{}",number);
    }
}

fn print_pharse(phase: &str) {
    println!("{}", phase)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}
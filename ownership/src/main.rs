fn main() {
    println!("Hello, world!");

    let var = 1;
    let mut s = "hello".to_string();

    s.push_str(" world");

    println!("{}",s);

    let x = vec!["karthick".to_string()];
    println!("{:?}",x);

    // Cannot move vale from x to y

    // let y = x;
    // println!("{:?}",x);
    // println!("{:?}",y);
    // let z =y;
    //
    // println!("{:?}",x);
    // println!("{:?}",y);
    // println!("{:?}",z);


    // CLONE

    let y = x.clone();
    let z = y.clone();

    println!("{:?}",x);
    println!("{:?}",y);
    println!("{:?}",z);

    // COPY
    // copy is going to implemented on types which is already stored on Stack such integer,boolean
    // float, tuple, char

    let a = 1;
    let b = a;

    println!("{}",a);
    println!("{}",b);

    // MORE MOVES
    let s1 = String::from("owner");
    take_ownership(s1);

    // println!("{}",s1); // cannot take ownership

    let num:i32 = 1;
    make_copy(num);
    println!("{}",num); // because by default integer having a copy trait

    let str2 = give_ownership();
    println!("{}",str2);

    let str4 = take_and_give(str2);
    println!("{}",str4);

    // REFERENCE

    let mut refs = String::from("hello");
    change_string(&mut refs);
    println!("{}",refs)



}

fn change_string(some_string : &mut String){
    some_string.push_str(", world!")
}

fn take_ownership(s:String){
    let strs = s;
    println!("{}",strs);
}

fn make_copy(n:i32){
    let n1 = n;
    println!("{}",n1)
}

fn give_ownership() -> String {
    "Karthick".to_string()
}

fn take_and_give(str3:String) -> String{
    str3
}
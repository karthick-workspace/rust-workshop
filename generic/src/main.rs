fn f<T>(ch: char, num1: T, num2: T) -> T {
    if ch == 'a' { num1 } else { num2 }
}

#[allow(dead_code)]
struct S<T1, T2> {
    c: char,
    n1: T1,
    n2: T1,
    n3: T2,
}

struct SE<T1, T2> (char, T1, T1, T2);

fn swap<T1, T2>(a: T1, b: T2) -> (T2, T1) { (b, a) }

fn main() {
    println!("Hello, world!");

    let a: i16 = f::<i16>('a', 23, 45);
    let b: f64 = f::<f64>('b', 45.4, 41.1);

    println!("{} , {}", a, b);

    // Inferring the Parametric Types
    let c = f('a', 23, 45);
    let d = f('b', 45.4, 41.1);

    println!("{} , {}", c, d);

    let _s = S {
        c: 'a',
        n1: 34,
        n2: 382,
        n3: 0.02,
    };

    println!("{}", _s.n3);

    let _se = SE('a', 34, 235, 0.02);

    println!("{}", _se.1);

    let x = swap(2i16, 4u16);
    let y = swap(5f32, true);

    println!("{:?} {:?}", x, y)
}


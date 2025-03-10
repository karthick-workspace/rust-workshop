// struct Point<T, U> {
//     x: T,
//     y: U,
// }
//
// fn main() {
//     let cord = Point { x: 1, y: 2.3 };
//     println!("Hello, world!");
// }

trait Overview {
    fn overview(&self) -> String {
        String::from("This is from RUST course")
    }
}

struct Course {
    headline: String,
    author: String,
}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for Course {
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}

impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}

fn callOveriew(overview: &impl Overview) {
    println!("Overview: {}", overview.overview())
}

fn callOveriewGeneric<T: Overview>(overview: T) {
    println!("Overview: {}", overview.overview())
}

fn main() {
    let course1 = Course { headline: "Headline!".to_string(), author: "Tyler".to_string() };
    let course2 = AnotherCourse { headline: "Another Headline!".to_string(), author: "Another Tyler".to_string() };

    println!("{}", course1.overview());
    println!("{}", course2.overview());

    callOveriew(&course1);
    callOveriew(&course2);

    callOveriewGeneric(course1);
    callOveriewGeneric(course2);
}


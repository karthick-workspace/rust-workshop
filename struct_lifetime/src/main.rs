struct User{
    active:bool,
    username:String,
    sign_in_count:u32,
}

struct Coordinates(i32,i32,i32);

struct UnitStruct;

struct Square {
    width: u32,
    height: u32,
}

impl Square{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn scale(&mut self,scale_factor:u32) {
        self.width *= scale_factor;
        self.height *= scale_factor;
    }
}

fn main() {
    println!("Hello, world!");

    let user1 = User{active:true,username:String::from("karthick"),sign_in_count:0};
    println!("{}",user1.username);

    let user2:User = build_user(String::from("Karthick"));
    println!("{}",user2.username);

    let cords = Coordinates(1,2,3);
    println!("{}",cords.0);

    let mut sq = Square{
        width: 2,
        height: 4
    };

    sq.scale(2);

    println!("{}",sq.area())
}

fn build_user(strs: String ) -> User {
    User{active:true,username:String::from(strs),sign_in_count:0}
}

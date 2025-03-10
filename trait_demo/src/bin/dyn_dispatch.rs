trait Human {
    fn get_name(&self) -> String;
}

struct Adult(String);

impl Human for Adult {
    fn get_name(&self) -> String {
        self.0.to_string()
    }
}

struct Child(String);

impl Human for Child {
    fn get_name(&self) -> String {
        self.0.to_string()
    }
}

fn create_person(adult: bool, name: String) -> Box<dyn Human> {
    if adult {
        Box::new(Adult(name))
    } else {
        Box::new(Child(name))
    }
}

fn main() {
    let adult = create_person(true, "karthick".to_owned());
    println!("{}", adult.get_name())
}

pub struct User {
    name: String,
    age: i32,
}

impl User {
    fn getName(&self) -> String {
        self.name.clone()
    }

    fn getAge(&self) -> i32 {
        self.age
    }
}

fn main() {
    let user1 = User {
        name: String::from("ABC"),
        age: 10,
    };

    println!("{}", user1.getName());
    println!("{}", user1.getAge());
}

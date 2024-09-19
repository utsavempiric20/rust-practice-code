fn main() {
    let user = User {
        name: String::from("ABCDEFG"),
        age: 25,
    };
    notify(&user);
    println!("{}", user.summrise());
}

pub trait Summary {
    fn summrise(&self) -> String {
        return String::from("Default");
    }
}

pub trait Car {
    fn cars(&self);
}

struct User {
    name: String,
    age: i32,
}

impl Summary for User {
    fn summrise(&self) -> String {
        return format!("This is {} and {} year old", self.name, self.age);
    }
}

impl Car for User {
    fn cars(&self) {
        println!("This is Car");
    }
}

// fn notify(item: &impl Summary) {
//     println!("notify : {}", item.summrise());
// }

fn notify<T: Summary + Car>(item: &T) {
    println!("notify : {}", item.summrise());
    item.cars();
}

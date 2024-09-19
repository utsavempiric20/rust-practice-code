pub trait Printable {
    fn print_info(&self);
}

struct Book {
    name: String,
    price: i32,
}

struct Car {
    name: String,
    price: i32,
}

impl Printable for Book {
    fn print_info(&self) {
        println!("Name : {} Price : {} ", self.name, self.price)
    }
}

impl Printable for Car {
    fn print_info(&self) {
        println!("Name : {} Price {} ", self.name, self.price);
    }
}

fn main() {
    let car = Car {
        name: String::from("Gwagon"),
        price: 800,
    };
    car.print_info();
}

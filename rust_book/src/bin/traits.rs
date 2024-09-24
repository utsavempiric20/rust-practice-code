trait Car {
    fn car_name(&self);
}

struct Factory {
    name: String,
}

impl Car for Factory {
    fn car_name(&self) {
        println!("{}", self.name);
    }
}

fn main() {
    let fac = Factory {
        name: String::from("Gwagon"),
    };
    fac.car_name();
}

struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    fn area(&self) {
        println!("area height : {} and width : {}", self.height, self.width);
    }
}

fn main() {
    let rec1 = Rectangle {
        height: 50,
        width: 40,
    };

    rec1.area();
}

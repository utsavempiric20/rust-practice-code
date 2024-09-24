pub struct Apple {
    name: String,
}

pub fn get_apple(name: String) -> String {
    let a1 = Apple { name };
    println!("garden : {}", a1.name);
    a1.name
}

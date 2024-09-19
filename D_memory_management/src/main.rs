// Ownership & Moving

fn main() {
    let s1 = String::from("empiric");
    let s2 = s1;

    // println!("{}", s1);  --> borrow of moved value
    println!("{}", s2);
}

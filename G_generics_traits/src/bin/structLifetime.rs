struct User<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: i32,
}

fn main() {
    let user;
    let f_name = String::from("ABC");
    {
        let last_name = String::from("DEF");
        user = User {
            first_name: &f_name,
            last_name: &last_name,
            age: 50,
        };
        println!("{} {}", user.first_name, user.last_name);
    }
    // println!("{} {}", user.first_name, user.last_name); //--> Err : `last_name` does not live long enough
}

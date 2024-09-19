use std::fs;

fn main() {
    // let result = fs::read_to_string("a.txt");
    let result = read_file(String::from("a.txt"));

    match result {
        Ok(data) => println!("{}", data),
        Err(err) => println!("{}", err),
    }
}

// Custom function
fn read_file(path: String) -> Result<String, String> {
    let result: Result<String, std::io::Error> = fs::read_to_string(path);

    match result {
        Ok(data) => Ok(data),
        Err(_err) => Err(String::from("Not found")),
    }
}

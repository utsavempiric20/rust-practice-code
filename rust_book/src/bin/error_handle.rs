use std::fs::File;

fn main() {
    // Handle using result
    let file = File::open("abc.txt");

    match file {
        Ok(file) => println!("{:?}", file),
        Err(err) => println!("File not found : {err}"),
    }

    // panic! and  stop
    let v1 = vec![1, 2];
    println!("{:?}", v1[5]);
}

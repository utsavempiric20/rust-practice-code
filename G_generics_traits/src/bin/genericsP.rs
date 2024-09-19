fn main() {
    let largest_num1 = largest_data(5, 1);
    let largest_num2 = largest_data("t", "d");
    println!("{}", largest_num1);
    println!("{}", largest_num2);
}

fn largest_data<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

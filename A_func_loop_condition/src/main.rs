fn main() {
    let str = String::from("utsav");
    println!("{}", get_string_count(str));
}

fn fibonacci(num: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;

    if num == 0 {
        return 0;
    }
    if num == 1 {
        return 1;
    }

    for _ in 0..num - 1 {
        let temp = b;
        b = a + b;
        a = temp;
    }
    return b;
}

fn get_string_count(str: String) -> usize {
    str.chars().count()
}

fn main() {
    let int_num = larest_num(88, 10);
    let floar_num = larest_num(8.4, 9.6);
    println!("{}", int_num);
    println!("{}", floar_num);
}

fn larest_num<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

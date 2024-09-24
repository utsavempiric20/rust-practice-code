fn main() {
    // let s1 = String::from("ABCD");
    // let (s2, len) = func_length(s1);
    // println!("{} {}", s2, len);

    let mut s1 = String::from("ABCD");
    let r1 = &s1;
    let r2 = &s1;
    println!("{} {}", r1, r2);

    let r3 = &mut s1;
    println!("{}", r3);
}

fn func_length(s: String) -> (String, usize) {
    let s1 = s.len();

    (s, s1)
}

fn main() {
    let s1 = String::from("ABCDFGHI");
    let word = &s1[0..3];

    // s1.clear();

    println!("{}", word);
    let s2 = first_word(&s1);
    let s3 = &s1[0..5];
    println!("{}", s2);
    println!("{}", s3);
}

fn first_word(s: &str) -> usize {
    let s1 = s.as_bytes();
    for (i, &item) in s1.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

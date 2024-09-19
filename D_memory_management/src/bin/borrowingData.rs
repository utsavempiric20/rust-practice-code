// fn main() {
//     let mut s1 = String::from("abc");
//     s2_own(&mut s1);

//     println!("{}", s1);
// }

// fn s2_own(s2: &mut String) {
//     s2.push_str("defgh");
//     println!("{}", s2);
// }

fn main() {
    let mut s1 = String::from("ABCD");
    let s2 = &mut s1;

    // println!("{}", s1); // cannot borrow `s1` as immutable because it is also borrowed as mutable immutable borrow occurs here
    println!("{}", s2);
}

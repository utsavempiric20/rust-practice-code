fn main() {
    let name = String::from("Abc Def Ghi");
    let first_word = &name[0..3];

    println!("{}", first_word);
}

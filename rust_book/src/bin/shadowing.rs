fn main() {
    let x = 1;

    let x = x + 2;
    {
        let x = x * 5;
        println!("{}", x);
    }
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);
    println!("{}", x);
}

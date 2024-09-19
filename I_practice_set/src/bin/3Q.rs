fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    check_ownership(&v1);
    println!("{:?}", v1);
}

fn check_ownership(vec: &Vec<i32>) {
    for i in vec {
        println!("{}", i)
    }
}

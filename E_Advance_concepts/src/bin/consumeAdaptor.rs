//  Consumeing Adaptors
fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();

    let v1_iter_sum: i32 = v1_iter.sum();

    println!("{}", v1_iter_sum);

    // let v1_iter_sum2: i32 = v1_iter.sum();  --> Err : use of moved value: `v1_iter`

    println!("{:?}", v1);
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();

    let filter_map_data = v1_iter.map(|x| x * 2).filter(|x| x % 2 == 1);
    println!("{:?}", v1);

    let v2: Vec<i32> = filter_map_data.collect();
    println!("{:?}", v2);
}

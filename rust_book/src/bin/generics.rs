fn biggest_num<T: std::cmp::PartialOrd>(nums: &[T]) -> &T {
    let mut largest_num = &nums[0];
    for i in nums {
        if i > largest_num {
            largest_num = i;
        }
    }
    largest_num
}

fn main() {
    let v1 = vec![5, 6, 4, 10, 16];
    let b = biggest_num(&v1);
    println!("{:?}", b);

    let v2 = vec![2.0, 50.0, 4.0, 20.0];
    let c = biggest_num(&v2);
    println!("{:?}", c);
}

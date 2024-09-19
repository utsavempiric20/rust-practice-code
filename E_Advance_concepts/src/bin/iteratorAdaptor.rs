// iteratorAdaptors map,filter

use std::vec;

fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let v1_iter = v1.iter();

    // let v1_iter_map = v1_iter.map(|x| x + 1);
    let v1_iter_filter = v1_iter.filter(|x| *x % 2 == 0).map(|x| x * 2);

    let vec: Vec<i32> = v1_iter_filter.collect();

    println!("{:?}", vec);

    println!("{:?}", v1);
}

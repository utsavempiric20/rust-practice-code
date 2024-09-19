// #1 : Normal Iterator but similar to num.iter()
// fn main() {
//     let num = vec![1, 2, 3, 4];
//     for val in num {
//         println!("{}", val);
//     }
// }

// #2 : Immutable iter
// fn main() {
//     let num = vec![1, 2, 3, 4];
//     let ans = num.iter();

//     for val in ans {
//         println!("{}", val);
//     }
//     println!("{:?}", num);
// }

// #3 : Mutable iter
// fn main() {
//     let mut num = vec![1, 2, 3, 4];
//     let ans = num.iter_mut();

//     for val in ans {
//         *val = *val + 1;
//     }

//     println!("{:?}", num);
// }

// #4 : Into_iter which is similar to #1
// fn main() {
//     let num = vec![1, 2, 3, 4];
//     let ans = num.into_iter();

//     for val in ans {
//         println!("{}", val);
//     }

//     // println!("{:?}", num); --> Err : borrow of moved value: `num
// }

// #5 : Mutable iter using .next
fn main() {
    let num = vec![1, 2, 3, 4];
    let mut ans = num.iter();

    while let Some(val) = ans.next() {
        println!("{}", val)
    }
}

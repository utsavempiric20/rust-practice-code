// Approach 1
// fn main() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);

//     let ans = find_even(&vec);
//     println!("{:?}", ans);
//     println!("{:?}", vec);
// }

// fn find_even(vec: &Vec<i32>) -> Vec<i32> {
//     let mut new_vec = Vec::new();
//     for val in vec {
//         if val % 2 == 0 {
//             new_vec.push(*val);
//         }
//     }
//     return new_vec;
// }

fn main() {
    // let mut vec = Vec::new();
    // vec.push(1);
    // vec.push(2);
    // vec.push(3);
    // vec.push(4);
    // vec.push(5);
    // vec.push(6);
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    find_even2(&mut vec);
    println!("{:?}", vec)
}

fn find_even2(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}

//Wihtout Option
// fn main() {
//     let ans = find_char(String::from("g-wagon"));
//     if ans == -1 {
//         println!("undfiend");
//     } else {
//         println!("index {}", ans)
//     }
// }

// fn find_char(str: String) -> i32 {
//     for (index, char) in str.chars().enumerate() {
//         if char == 'a' {
//             return index as i32;
//         }
//     }
//     return -1;
// }

// With option
fn main() {
    let ans = find_char(String::from("g-wgon"));
    match ans {
        Some(data) => println!("{}", data),
        None => println!("Not found"),
    }
}

fn find_char(str: String) -> Option<i32> {
    for (index, char) in str.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

use std::collections::HashMap;

// fn main() {
//     let mut user: HashMap<String, u32> = HashMap::new();
//     user.insert(String::from("A"), 1);
//     user.insert(String::from("B"), 2);

//     let user2 = user.get("C");

//     println!("{:?}", user);
//     match user2 {
//         Some(num) => println!("{}", num),
//         None => println!("user not found"),
//     }
// }

fn main() {
    let vec = vec![(String::from("ABC"), 1), (String::from("DEF"), 2)];
    let hm = group_by_key_values(vec);

    println!("{:?}", hm);
}

fn group_by_key_values(vec: Vec<(String, u32)>) -> HashMap<String, u32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    return hm;
}

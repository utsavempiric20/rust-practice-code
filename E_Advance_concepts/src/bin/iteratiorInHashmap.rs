use std::{collections::HashMap, vec};

fn main() {
    let v1 = vec![(String::from("ABC"), 20), (String::from("DEF"), 21)];
    let new_hm: HashMap<String, i32> = vec_to_hashmap(v1);
    let new_hm_iter = new_hm.iter();

    let new_vec: Vec<(&String, &i32)> = new_hm_iter.map(|(k, v)| (k, v)).collect();
    println!("{:?}", new_vec)
}

fn vec_to_hashmap(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    return hm;
}

use std::collections::HashMap;

fn main() {
    let mut student_data = HashMap::new();
    student_data.insert(String::from("ABC"), 50);
    student_data.insert(String::from("DEF"), 20);
    student_data.insert(String::from("XYZ"), 95);

    let total: i32 = student_data.values().sum();
    let count = student_data.len();
    let average_score = total as f32 / count as f32;
    println!("{}", average_score);

    if let Some(max) = student_data.values().max() {
        println!("{}", max);
    } else {
        println!("There is no score available");
    }
}

fn main() {
    let word = String::from("Gwagon Car");
    // let word2 = part_of_string_1(word);
    // let word2 = part_of_string_2(&word);
    let word2 = &word[0..5];

    println!("{}", word2)
}

fn part_of_string_1(word: String) -> String {
    let mut new_str = String::from("");

    for i in word.chars() {
        if i == ' ' {
            break;
        }
        new_str.push(i);
    }
    return new_str;
}

fn part_of_string_2(word: &String) -> &str {
    let mut index = 0;
    for i in word.chars() {
        if i == 'r' {
            break;
        }
        index = index + 1;
    }
    return &word[0..index];
}

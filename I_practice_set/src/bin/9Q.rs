fn main() {
    let mut ans;
    let a1 = String::from("avc");
    {
        let a2 = String::from("erty");
        ans = long_string(&a1, &a2);
        // println!("big : {}", ans);
    }

    let a3 = String::from("yt");
    ans = small_string(&a1, &a3);
    println!("small : {}", ans);
}

fn long_string<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}

fn small_string<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() < b.len() {
        return a;
    } else {
        return b;
    }
}

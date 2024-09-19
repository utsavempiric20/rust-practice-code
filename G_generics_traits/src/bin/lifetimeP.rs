// Lifetime ensure that the references do not outlive the data they point to, prevent us to dangling pointer and memory saftey issue.

fn main() {
    let ans;

    let str1 = String::from("smagfgfgll");
    {
        let str2 = String::from("longeraaatryuiop");
        ans = longest(&str1, &str2);
        println!("{}", ans);
    }
    // println!("{}", ans);  --> Err : `str2` does not live long enough
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}

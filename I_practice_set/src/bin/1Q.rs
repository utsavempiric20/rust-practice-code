fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = sum_of_even(&v1);
    let v3 = even_vec(v1);
    println!("{}", v2);
    println!("{:?}", v3);
}

fn even_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut v1 = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            v1.push(val);
        }
    }
    return v1;
}

fn sum_of_even(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for val in vec {
        if val % 2 == 0 {
            sum = sum + val;
        }
    }
    return sum;
}

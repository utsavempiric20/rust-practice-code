use std::thread;

fn main() {
    let x = 1;
    {
        let v = vec![1, 2, 3, 4];

        thread::spawn(move || {
            println!("{:?}", v);
        });
    }
    println!("{}", x);
}

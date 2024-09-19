//  Transfer data between the threads

use std::{sync::mpsc, thread::spawn};

// fn main() {
//     let (tx, rx) = mpsc::channel::<String>();

//     thread::spawn(move || tx.send(String::from("ABC")));

//     let value = rx.recv();
//     match value {
//         Ok(data) => println!("{}", data),
//         Err(err) => println!("Does not recieve data {}", err),
//     }
// }

fn main() {
    let (tx, rx) = mpsc::channel::<u64>();

    for i in 0..10 {
        let producer = tx.clone();
        spawn(move || {
            let mut sum = 0;
            for j in i * 10000000..(i + 1 * 10000000) {
                sum = sum + j;
            }
            producer.send(sum)
        });
    }
    drop(tx);

    let mut final_sum = 0;
    for val in rx {
        final_sum = final_sum + val;
    }
    println!("final sum {}", final_sum);
}

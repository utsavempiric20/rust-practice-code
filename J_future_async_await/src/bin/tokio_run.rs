use tokio::spawn;

async fn read_string(data: String) -> String {
    return data;
}

async fn my_function(i: i32) {
    println!("Start fetching...");
    let s1 = read_string(String::from("User1")).await;
    println!("{i} {s1}");

    let s2 = read_string(String::from("User2")).await;
    println!("{i} {s2}");
}

#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for i in 0..2 {
        let handle = spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    for val in handles {
        val.await.unwrap();
    }
}

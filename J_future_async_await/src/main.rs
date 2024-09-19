use futures::executor::block_on;

async fn checking_user_data_india() {
    let user1 = user_data(String::from("user1 IND")).await;
    println!("{}", user1);
    let user2 = user_data(String::from("user2 IND")).await;
    println!("{}", user2);
    let user3 = user_data(String::from("user3 IND")).await;
    println!("{}", user3);
}

async fn checking_user_data_us() {
    let user1 = user_data(String::from("user1 US")).await;
    println!("{}", user1);
}

async fn user_data(name: String) -> String {
    return name;
}

async fn async_main() {
    let a1 = checking_user_data_india();
    let a2 = checking_user_data_us();

    futures::join!(a1, a2);
}

fn main() {
    block_on(async_main());
}

// enum AutoMobile {
//     Car(String, u32),
//     Bike(String, u32),
//     Cycle,
// }

// fn check_automobile_stocks(automobile: AutoMobile) {
//     match automobile {
//         AutoMobile::Bike(name, price) => println!("{} {}", name, price),
//         AutoMobile::Car(name, price) => println!("{} {}", name, price),
//     }
// }

// fn main() {
//     check_automobile_stocks(AutoMobile::Car(String::from("Gwagon"), 4));
//     check_automobile_stocks(AutoMobile::Bike(String::from("Suzuxi Ninza"), 2));
// }

#[derive(Debug)]
enum Usgovernment {
    A1,
    A2,
}

enum Coin {
    Inr,
    Quarter(Usgovernment),
}

fn main() {
    // dice_roll(6);
    let val = Coin::Quarter(Usgovernment::A1);
    if let Coin::Quarter(state) = val {
        println!("{:?}", state);
    } else {
        println!("out");
    }
}

fn dice_roll(num: u32) {
    match num {
        2 => println!("Stop"),
        6 => println!("Leave"),
        _ => println!("Move"),
    }
}

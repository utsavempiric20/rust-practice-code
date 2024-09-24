mod garden;

mod house {
    pub struct Breakfast {
        pub toast: String,
        fruit: String,
    }

    impl Breakfast {
        pub fn breakfast_summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: toast.to_string(),
                fruit: String::from("Apples"),
            }
        }
    }
}

use house::Breakfast;

fn main() {
    let a = Breakfast::breakfast_summer("tie");

    let b = garden::get_apple(String::from("berry"));
    println!("{}", b);
    println!("{:?}", a.toast);
}

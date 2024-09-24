fn main() {
    let mut counter = 0;
    'loop1: loop {
        if counter == 10 {
            break;
        }
        loop {
            if counter == 3 {
                break 'loop1;
            }
            counter = counter + 1;
        }
        println!("Hello");
        counter = counter + 1;
    }
}

fn main() {
    println!("Hello, world!");
    //Options
    println!("{:?}", find_square_root(3.0));
}

fn find_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

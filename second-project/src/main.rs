fn main() {
    println!("Hello, world!");
    //Options
    println!("{:?}", find_square_root(3.0));
    let number = -4.0;
    let square_root = find_square_root(number);
    match square_root {
        Some(_value) => println!("Inside some"),
        None => println!("Inside none"),
    }
}

fn find_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

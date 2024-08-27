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
    println!("{:?}", divide(1.0, 0.0));
    println!("{:?}", divide(1.0, 2.0));
    let division_result = divide(1.0, 2.0);
    match division_result {
        Err(error_msg) => println!("{}", error_msg),
        Ok(value) => println!("{}", value),
    }
    let base = get_from_database("base");
    let height = get_from_database("height");
    let area_result = calculate_triangle_area(base, height);
    match area_result {
        Ok(value) => println!("Value is {}", value),
        Err(err_msg) => println!("{}", err_msg),
    }

    let mut_numbers = vec![1, 2, 3, 4];
    let mut names: Vec<String> = Vec::new();
    names.push(String::from("Alice"));
    names.push(String::from("Bob"));
    let name1 = &names[0];
    let name2 = &names[1];
    println!("{},{}", name1, name2);
    for num in &mut_numbers {
        println!("{}", num);
    }

    let slice = &mut_numbers[1..2];
    println!("{:?}", slice);

    //Strings are also a collection
    let mut my_string = String::from("my");
    let mut my_second_string = String::from("Second String");
    my_string.push_str("string");
    println!("{}", my_string);

    for c in my_string.chars() {
        println!("{}", c)
    }
    for b in my_string.bytes() {
        println!("{}", b)
    }
}

fn find_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(a / b)
    }
}

fn get_from_database(key: &str) -> Option<f64> {
    let database: [(&str, Option<f64>); 2] = [("base", Some(4.0)), ("height", Some(6.0))];
    for (k, v) in database {
        if k == key {
            return v;
        }
    }
    None
}

fn calculate_triangle_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String> {
    match (base, height) {
        (Some(b), Some(h)) => {
            if b <= 0.0 || h <= 0.0 {
                Err("Both base and height should be positive numbers".to_string())
            } else {
                Ok(0.5 * b * h)
            }
        }
        (None, _) => Err("Base is missing".to_string()),
        (_, None) => Err("Height is missing".to_string()),
    }
}

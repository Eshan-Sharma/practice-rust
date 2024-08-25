fn main() {
    println!("Hello, world!");
    let _message = "Hello world!";
    let _x: i32 = 32;
    let _pi: f64 = 3.14;
    let _is_rust_fun: bool = true;
    let _letter_a: char = 'a';

    fn _add(x: i32, y: i32) -> i32 {
        //return x + y;
        x + y
    }

    let x = 42; //shadowing
    if x > 0 {
        println!("x is not a negative");
    } else {
        println!("x is negative");
    }

    let mut i = 1;
    while i <= 5 {
        println!("{}", i);
        i = i + 1;
    }

    //Data types
    let _my_first_bool = true;
    let _my_second_bool = false;
    let _days_of_week: i8 = 7;
    let _number_of_users: i128 = 234234;
    let _number_of_tokens: u128 = 34234; //unsigned
    let _string_message: &str = "Hi, Person";
    let _second_string: String = String::from("Hello another person");
    //Arrays
    let _days_of_week: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    let _first_day = _days_of_week[0];
    let _last_day = _days_of_week[_days_of_week.len() - 1];
    println!("{},{}", _first_day, _last_day);
    //Slice
    let slice = &_days_of_week[1..3];
    println!("{:?}", slice);
    //Tuple
    let person = ("Alex", 120);
    println!("{}", person.0);
    println!("{}", person.1);
    //Unit Type
    let _unit_type = ();

    //Variables
    let mut num = 5; //Mutability
    num = 4;
    println!("{}", num);
}

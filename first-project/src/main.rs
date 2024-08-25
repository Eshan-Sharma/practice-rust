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
    println!("{}", num);
    num = 4;
    println!("{}", num);

    println!("Sum is {}", add(1, 2));

    if _days_of_week[6] == "Sunday" {
        println!("The race day!");
    } else if _days_of_week[5] == "Saturday" {
        println!("The qualifying day");
    } else {
        println!("Patiently wait for the race day");
    }

    //While
    let mut counter = 0;
    while counter < 5 {
        println!("Counter {}", counter);
        counter += 1;
    }

    //For loop
    for number in 1..5 {
        println!("Number is {}", number);
    }
    //For loop
    for number in 1..=5 {
        println!("Numbers again is {}", number);
    }
    //loop
    let mut counter = 1;
    loop {
        println!("Counter value is {}", counter);
        counter += 1;
        if counter == 6 {
            println!("Breaks");
            break;
        }
    }
    //Match
    let new_num = 5;
    match new_num {
        1 => println!("Number is one"),
        2 => println!("Number is two"),
        3 => println!("Number is three"),
        4 => println!("Number is four"),
        5 => println!("Number is five"),
        _ => println!("Numebr is something else"),
    }
    //Match results
    let result = match new_num {
        1 => "Number is one",
        2 => "Number is two",
        3 => "Number is three",
        4 => "Number is four",
        5 => "Number is five",
        _ => "Number is something else",
    };
    println!("Result is '{}'", result);

    //Ownership
    let _s1 = String::from("Practice ownership");
    let _s2 = _s1;
    //println!("{}", _s1.len()); //This won't work, s1 is moved to s2. Ownership has been transferred

    //References
    let my_string = String::from("Hello world");
    let my_ref = &my_string;
    println!("{}", my_ref);
    print_string(&my_string);
    println!("I still got my string {}", my_string);

    //Mutable Reference
    let mut mut_string = String::from("Hello");
    change_string(&mut mut_string);
    println!("{}", mut_string);

    //We cannot have mutable immutable reference together if we are going to use immutable reference from now on
    let _first_mut_reference = &mut mut_string;
    let _second_mut_reference = &mut_string;
    //println!("{}", _first_mut_reference); //Does not work
}
fn print_string(s: &String) {
    println!("{}", s);
}
fn change_string(s: &mut String) {
    s.push_str(" World");
}
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let first_string = String::from("HEllo");
    let second_string = String::from("World");
    println!("{}", concatenate_strings(&first_string, &second_string));
}

fn concatenate_strings(str1: &String, str2: &String) -> String {
    let new_string = String::from(str1.clone() + str2);
    new_string
}

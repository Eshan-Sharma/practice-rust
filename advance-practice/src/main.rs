fn main() {
    let dog = Dog {
        name: "Rodolf".to_string(),
    };
    dog.speak();
}

trait Speak {
    fn speak(&self);
}

struct Dog {
    name: String,
}
impl Speak for Dog {
    fn speak(&self) {
        println!("{}. says:Woof!", self.name);
    }
}

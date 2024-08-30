#[tokio::main]
async fn main() {
    println!("Hello, world!");
    
   let f =  my_function();
   println!("Before f");
   f.await;
}

async fn my_function(){
    println!("I'am an async function");
    let s1:String = read_from_database().await;//without await it will return a future
    println!("First result: {}",s1);
    let s2:String = read_from_database().await;
    println!("Second result: {}",s2);

}

async fn read_from_database() -> String{
    "DB Result".to_owned()
}

// fn my_function_without_syntax_sugar -> impl Future<Output=()>{
//     println!("I am async function")
// }

//Futures are similar to promises in javascript except they are lazy and won't do anything unless driven to completion by being polled.
//This makes Futures a zero cost abstraction, they can be driven to completion by awaiting them or giving it to an executor.
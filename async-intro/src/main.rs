use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
//     println!("Hello, world!");
    
//    let f =  my_function();
//    println!("Before f");
//    f.await;
    let mut handles = vec![];
    for i in 0..2{
        let handle = tokio::spawn(async move{
            my_function(i).await;
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.await.unwrap();
    }
}

async fn my_function(i:i32){
    println!("I'am an async function");
    let s1:String = read_from_database().await;//without await it will return a future
    println!("[{i}]First result: {}",s1);
    let s2:String = read_from_database().await;
    println!("[{i}]Second result: {}",s2);

}

async fn read_from_database() -> String{
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_owned()
}

// fn my_function_without_syntax_sugar -> impl Future<Output=()>{
//     println!("I am async function")
// }

//Futures are similar to promises in javascript except they are lazy and won't do anything unless driven to completion by being polled.
//This makes Futures a zero cost abstraction, they can be driven to completion by awaiting them or giving it to an executor.
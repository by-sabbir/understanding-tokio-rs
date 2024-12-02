use tokio::{join, spawn};

async fn hello() {
    println!("hello")
}

async fn test2() {
    println!("second task")
}
async fn test1() {
    println!("first task")
}

async fn counter() {
    for i in 0..10 {
        println!("{i}");
        tokio::task::yield_now().await;
    }
}

#[tokio::main]
async fn main() {
    hello().await;
    join!(test1(), test2());

    let _ = join!(spawn(counter()), spawn(counter()),);
}

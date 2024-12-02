use std::time::Duration;

use tokio::{join, task::spawn_blocking};

async fn delayed_task(task: i32, time: u64) {
    println!("Spawned Task {task} took {time}");

    let _ = spawn_blocking(move || {
        std::thread::sleep(Duration::from_millis(time));
    })
    .await;

    println!("Finished Task {task} took {time}");
}

#[tokio::main]
async fn main() {
    join!(
        delayed_task(1, 500),
        delayed_task(2, 1000),
        delayed_task(3, 1500),
    );
}

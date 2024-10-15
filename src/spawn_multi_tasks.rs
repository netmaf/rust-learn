use tokio::time::{sleep, Duration};
use tokio::join;

async fn task_one() {
    println!("Task One started");
    sleep(Duration::from_secs(2)).await;
    println!("Task One completed after 2 seconds");
}

async fn task_two() {
    println!("Task Two started");
    sleep(Duration::from_secs(3)).await;
    println!("Task Two completed after 3 seconds");
}

async fn task_three() {
    println!("Task Three started");
    sleep(Duration::from_secs(1)).await;
    println!("Task Three completed after 1 second");
}

#[tokio::main]
async fn main() {
    println!("Starting all tasks...");

    // Run all tasks concurrently and wait for all to complete
    join!(
        task_one(),
        task_two(),
        task_three()
    );

    println!("All tasks have completed");
}

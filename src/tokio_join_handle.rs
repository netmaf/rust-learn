// spawn a new task using tokio and handle returned value by async block


#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        return "Hello";
    });

    let out = handle.await.unwrap();

    println!("{out}");
}

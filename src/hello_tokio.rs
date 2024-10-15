async fn say() {
    println!("Hello")
}

#[tokio::main]
async fn main() {
    let op = say();
    println!("Hello, world 1!");
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);
    op.await;
}

use std::{sync::mpsc::channel, thread::spawn};

fn main() {
    let (tx, rx) = channel();
    spawn(move || {
        tx.send(String::from("sunny"))
    });

    let received = rx.recv();
    println!("Received {}", received.unwrap());
}

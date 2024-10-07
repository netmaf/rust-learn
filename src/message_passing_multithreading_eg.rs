use std::{sync::mpsc::channel, thread::spawn};

fn main() {
    let (tx, rx) = channel();

    // find the sum from 1 to 10^9

    for i in 0..10 {
        let tx_copy = tx.clone();
        spawn(move || {
            let mut ans: u64 = 0;
            for j in 1..10000001 {
                ans = ans + i * 10000000 + j;
            }
            tx_copy.send(ans).unwrap();
        });
    } 

    drop(tx); // drop original transmitter else receiver will keep waiting to get get all transmitters out of scope
    let mut sum: u64 = 0;
    for val in rx {
        sum = sum + val;
    }

    println!("Sum from 1 to 10^8 is -> {}", sum)
}

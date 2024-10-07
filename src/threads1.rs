// threads

use std::thread::spawn;

fn main() {
    let handler = spawn(|| {
        let mut c = 0;
        for _ in 1..500000000 {
            for _ in 1..500000000 {
                c = c + 1;
                c = c - 1;
            }
        }
    });

    let mut c = 0;
    /*
        Total operations: 500,000,000 * 500,000,000 = 2.5 * 10^17 (250 quadrillion)
        Assuming a modern CPU that can perform about 4 billion operations per second (which is optimistic and assumes perfect conditions):
        Time estimate = 5 * 10^17 / (4 * 10^9) seconds
        ≈ 1.25 * 10^8 seconds
        ≈ 1,447,917 days
        ≈ 3,966 years
     */
    for _ in 1..500000000 {
        for _ in 1..500000000 {
            c = c + 1;
            c = c - 1;
        }
    }
    handler.join(); // wait for associated thread to finish

}

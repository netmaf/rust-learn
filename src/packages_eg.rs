use chrono::{Utc, Local};

fn main() {
    println!("Utc -> {}", Utc::now());
    println!("Local -> {}", Local::now());
}

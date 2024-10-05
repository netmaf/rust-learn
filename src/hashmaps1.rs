use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<String, i32> = HashMap::new();
    hm.insert(String::from("Sunny"), 10);
    hm.insert(String::from("Lokesh"), 20);
    println!("{:?}", hm);

    let sunnys_age = hm.get("Sunny");

    match sunnys_age {
        Some(val) => println!("{}", val),
        None => println!("Val not present")
    }
}

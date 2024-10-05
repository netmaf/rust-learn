use std::collections::HashMap;

fn fun1(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut ans: HashMap<String, i32> = HashMap::new();

    for (p1, p2) in vec {
        ans.insert(p1, p2);
    }

    return ans;
}

fn main() {
    let vec = vec![(String::from("Sunny"), 10), (String::from("Lokesh"), 20), (String::from("Lokesh"), 21)];

    println!("{:?}", fun1(vec));
}

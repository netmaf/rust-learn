fn get_longest_string<'a>(w1: &'a str, w2: &'a str) -> &'a str { // 'a is generic lifetime annotation
    if w1.len() > w2.len() {
        return w1;
    }

    return w2;
}
fn main() {
    let longest_str;
    let w1 = String::from("Small");
    {
        let w2 = String::from("Longest");
        longest_str = get_longest_string(&w1, &w2);
        println!("{}", longest_str)
    }

    // println!("{}", longest_str) // compiler complains here

}

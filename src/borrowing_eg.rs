//there can be either 1 mutable reference or many immutable references
// &mut -> mutable refernece | & immutable refenrece
fn main() {
    let mut s1 = String::from("Sunny");
    let mut s2 = &mut s1;
    // fun1(&s1);
    fun2(&mut s2);

    println!("{}", s2)
}

fn fun1(s1: &String) {
    println!("{}", s1);
}

fn fun2(s1: &mut String) {
    s1.push_str(" S");
}
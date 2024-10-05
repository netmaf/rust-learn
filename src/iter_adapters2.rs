fn main() {
    let v = vec![1, 2, 3];
    let iter = v.iter();
    let iter2 = iter.map(|x| 2 * x);  // called as iterator adapter as it returns another adapter
    for el in iter2 {
        println!("{}",el)
    }

    println!();
    // filter example
    let v2 = vec![1, 2, 3, 4, 5];
    let iter = v2.iter();
    let filtered = iter.filter(|x| *x % 2 == 1); // called as iterator adapter as it returns another adapter
    let doubled = filtered.map(|x| 2 * x);

    let ans: Vec<i32> = doubled.collect(); // convert iterator into collection

    println!("{:?}", ans)

}

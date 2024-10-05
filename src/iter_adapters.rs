fn main() {
    let v = vec![1, 2, 3];
    let iter = v.iter(); // it consumer iterator (its a consuming adapter)
    let sum: i32= iter.sum();
    assert_eq!(sum, 6);
}

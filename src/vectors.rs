fn main() {
    let vec = vec![1, 2, 3];

    for num in vec {
        println!("{}", num)
    }
    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);
    vec.push(31);

    let ans = get_even(&vec);
    println!("{:?}", ans);
}

fn get_even(vec: &Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            ans.push(*val);
        }
    }

    return ans;
}

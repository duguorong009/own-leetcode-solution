fn main() {
    let n = 5;
    let integers = sum_zero(n);
    println!("{:?}", integers);
}

fn sum_zero(n: i32) -> Vec<i32> {
    let mut res = vec![];
    for i in 0..n / 2 {
        res.push(i + 1);
        res.push( -i -1);
    }
    if n % 2 == 1 {
        res.push(0);
    }
    res
}
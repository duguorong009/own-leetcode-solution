fn main() {
    println!("Hello, world!");
}

fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    for i in 1..10 {
        dfs(i, i, &mut res, low, high);
    }
    res.sort_unstable();
    res
}

fn dfs(last_digit: i32, cur: i32, all: &mut Vec<i32>, low: i32, high: i32) {
    if cur >= low && cur <= high {
        all.push(cur);
    }
    if cur >= high {
        return;
    }
    if last_digit < 9 {
        dfs(last_digit + 1, cur * 10 + last_digit + 1, all, low, high );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let low = 100;
        let high = 300;
        assert_eq!(sequential_digits(low, high), vec![123, 234]);
    }

    #[test]
    fn test_2() {
        let low = 1000;
        let high = 13000;
        assert_eq!(sequential_digits(low, high), vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]);
    }
}
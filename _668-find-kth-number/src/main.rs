fn main() {
    println!("Hello, world!");
}

fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
    let mut lo = 1;
    let mut hi = m * n;
    while lo < hi {
        let mi = lo + (hi - lo) / 2;
        if count(mi, m, n) < k {
            lo = mi + 1;
        } else {
            hi = mi;
        }
    }
    lo
}

fn count(x: i32, m: i32, n: i32) -> i32 {
    let mut res = 0;
    for i in 1..=m {
        res += n.min(x / i);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let m = 3;
        let n = 3;
        let k = 5;
        assert_eq!(find_kth_number(m, n, k), 3);
    }

    #[test]
    fn test_2() {
        let m = 2;
        let n = 3;
        let k = 6;
        assert_eq!(find_kth_number(m, n, k), 6);
    }
}

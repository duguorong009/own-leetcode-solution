fn main() {
    println!("Hello, world!");
}

struct Solution {
    bad: i32,
}

impl Solution {
    fn new(bad: i32) -> Self {
        Solution { bad }
    }

    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        version >= self.bad
    }
}

impl Solution {
    // Main fact is that there is constraint 1 <= bad <= n <= 2^31 -1.
    // Hence, when calculating mid, not (lower + upper)/2, but lower + (upper - lower) / 2.
    fn first_bad_version(&self, n: i32) -> i32 {
        let mut lower = 1;
        let mut upper = n;
        while lower < upper {
            let mid = lower + (upper - lower) / 2;
            if !self.isBadVersion(mid) {
                lower = mid + 1;
            } else {
                upper = mid;
            }
        }
        lower
    }
}

#[test]
fn test() {
    let bad = 4;
    let solution = Solution::new(bad);
    let n = 5;
    let res = 4;
    assert_eq!(solution.first_bad_version(n), res);
    let bad = 1;
    let solution = Solution::new(bad);
    let n = 1;
    let res = 1;
    assert_eq!(solution.first_bad_version(n), res);
}

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    fn digit_square_sum(mut x: i32) -> i32 {
        let mut sum = 0;
        while x > 0 {
            let tmp = x % 10;
            sum += tmp * tmp;
            x /= 10;
        }
        sum
    }

    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = n;
        loop {
            slow = Self::digit_square_sum(slow);
            fast = Self::digit_square_sum(fast);
            fast = Self::digit_square_sum(fast);
            if slow == fast {
                break;
            }
        }
        slow == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_happy(19));
        assert!(!Solution::is_happy(2));
    }
}
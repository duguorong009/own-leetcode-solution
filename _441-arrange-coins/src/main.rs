fn main() {
    let n = 5;
    assert_eq!(arrange_coins_using_binary_search(n), 2);
}

fn arrange_coins(n: i32) -> i32 {
    let mut n = n;
    if n <= 0 {
        return 0;
    }

    let mut count = 1;
    while n > 0 {
        n = n - count;
        if n < 0 {
            break;
        }
        count += 1;
    }
    count - 1
}

fn arrange_coins_using_binary_search(n: i32) -> i32 {
    let mut start = 1;
    let mut end = n;
    let mut mid = 0;
    let mut curr_sum = 0;
    while start <= end {
        // To prevent the overflow, not used mid calc using (start + mid) / 2
        mid = start + (end - start) / 2;
        curr_sum = mid * (mid + 1) / 2;
        if curr_sum == n {
            return mid;
        }

        if n < curr_sum {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    end
}

// In essence, use the formula solving
// (x + 1) * x / 2 <= n or
//  x^2 + x - 2n = 0
fn arrange_coins_most_simple_solution(n: i32) -> i32 {
    (((2 * n as i64) as f64 + 0.25).sqrt() - 0.5).floor() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrange_coins;

    #[test]
    fn test_1() {
        let n = 5;
        assert_eq!(arrange_coins(n), 2);
    }

    #[test]
    fn test_2() {
        let n = 8;
        assert_eq!(arrange_coins_most_simple_solution(n), 3);
    }
}

fn main() {
    println!("Hello, world!");
}

const MOD: usize = 1_000_000_007;
// Use the dynamic programming.
fn num_tiling(n: i32) -> i32 {
    let n = n as usize;
    if n == 0 {
        return 0;
    }
    let mut memo: Vec<usize> = vec![0; n + 1];
    dp(n, &mut memo) as i32
}

fn dp(n: usize, memo: &mut Vec<usize>) -> usize {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 5,
        _ => {
            if memo[n] > 0 {
                return memo[n];
            }
            let mut res = 0;
            res += dp(n - 3, memo);
            res %= MOD;
            res += 2 * dp(n - 1, memo);
            res %= MOD;
            memo[n] = res;
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(num_tiling(3), 5);
        assert_eq!(num_tiling(1), 1);
    }
}

use std::cmp::Reverse;

fn main() {
    println!("Hello, world!");
}

fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    let mut envelopes = envelopes;
    let n = envelopes.len();
    envelopes.sort_unstable_by_key(|v| (v[0], Reverse(v[1])));
    let mut dp = vec![];
    for i in 0..n {
        let height = envelopes[i][1];
        if let Err(p) = dp.binary_search(&height) {
            if p == dp.len() {
                dp.push(height);
            } else {
                dp[p] = height;
            }
        }
    }
    dp.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let envelopes = vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]];
        assert_eq!(max_envelopes(envelopes), 3);
    }

    #[test]
    fn test_2() {
        let envelopes = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
        assert_eq!(max_envelopes(envelopes), 1);
    }
}

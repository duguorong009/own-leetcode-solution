fn main() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    assert_eq!(length_of_lis(nums), 4);
}

// Essence of the solution is to apply DP

fn length_of_lis(nums: Vec<i32>) -> i32 {
    // // DP solution O(n^2).
    // if nums.len() == 1 {
    //     return 1;
    // }
    // let mut dp: Vec<i32> = vec![1; nums.len()];
    // for i in 0..nums.len() {
    //     for j in 0..i {
    //         if nums[j] < nums[i] {
    //             dp[i] = dp[i].max(dp[j] + 1);
    //         }
    //     }
    // }
    // dp.into_iter().max_by(|x, y| x.cmp(y)).unwrap()

    // Patience Sort.
    let mut piles: Vec<i32> = vec![];
    for num in nums {
        if piles.len() == 0 {
            piles.push(num);
            continue;
        }
        if num > piles[piles.len() - 1] {
            piles.push(num);
        } else {
            let pile_id = match piles.binary_search(&num) {
                Ok(i) => i,
                Err(i) => i,
            };
            piles[pile_id] = num;
        }
    }
    piles.len() as i32
}

#[cfg(test)]
mod tests {
    use crate::length_of_lis;

    #[test]
    fn test_1() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(length_of_lis(nums), 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(length_of_lis(nums), 4);
    }

    #[test]
    fn test_3() {
        let nums = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(length_of_lis(nums), 1);
    }
}

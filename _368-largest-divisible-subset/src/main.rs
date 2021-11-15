fn main() {
    println!("Hello, world!");
}

fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut index: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];
    let mut max_size = 1;
    nums.sort_unstable();
    for i in 0..n {
        for j in 0..i {
            if nums[i] % nums[j] == 0 && size[j] + 1 > size[i] {
                index[i] = j;
                size[i] = size[j] + 1;
                max_size = max_size.max(size[i]);
            }
        }
    }

    let mut res = vec![];
    for i in 0..n {
        if size[i] == max_size {
            let mut j = i;
            while index[j] != j {
                res.push(nums[j]);
                j = index[j];
            }
            res.push(nums[j]);
            break;
        }
    }
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use crate::largest_divisible_subset;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3];
        let res = vec![1, 2];
        assert_eq!(largest_divisible_subset(nums), res);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 4, 8];
        let res = vec![1, 2, 4, 8];
        assert_eq!(largest_divisible_subset(nums), res);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3, 10];
        let res = vec![1, 2, 10];
        assert_eq!(largest_divisible_subset(nums), res);
    }
}

fn main() {
    println!("Hello, world!");
}

fn find_min(nums: Vec<i32>) -> i32 {
    let mut unique_nums: Vec<i32> = vec![];
    for num in nums {
        if unique_nums.contains(&num) {
            continue;
        } else {
            unique_nums.push(num);
        }
    }
    let n = unique_nums.len();
    if n == 1 {
        return unique_nums[0];
    }
    let mut l = 0;
    let mut h = unique_nums.len() - 1;
    while l < h {
        if unique_nums[l] < unique_nums[h] {
            return unique_nums[l];
        }
        let m = l + (h - l) / 2;
        if unique_nums[l] <= unique_nums[m] {
            l = m + 1;
        } else {
            h = m;
        }
    }
    unique_nums[l]
}

#[cfg(test)]
mod tests {
    use crate::find_min;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 5];
        assert_eq!(find_min(nums), 1);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 2, 2, 0, 1];
        assert_eq!(find_min(nums), 0);
    }
}

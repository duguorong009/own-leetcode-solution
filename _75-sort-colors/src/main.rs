fn main() {
    println!("Hello, world!");
}

fn sort_color(nums: &mut Vec<i32>) {
    if nums.len() <= 1 {
        return;
    }
    let mut cnts: Vec<i32> = vec![0, 0, 0];
    for num in nums.into_iter() {
        cnts[*num as usize] += 1;
    }
    for i in 0..nums.len() {
        if i < cnts[0] as usize {
            nums[i] = 0;
        } else if i < (cnts[0] + cnts[1]) as usize {
            nums[i] = 1;
        } else {
            nums[i] = 2;
        }
    }
}

// Wonderful solution using double(maybe triple) pointers
fn sort_color_2(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut l = 0;
    let mut i = 0;
    let mut r = n - 1;
    while i <= r {
        while nums[i] == 2 && i < r {
            nums.swap(i, r);
            r -= 1;
        }
        while nums[i] == 0 && i > l {
            nums.swap(i, l);
            l += 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::sort_color;

    #[test]
    fn test_1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_color(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![2, 0, 1];
        sort_color(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![0];
        sort_color(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_4() {
        let mut nums = vec![1];
        sort_color(&mut nums);
        assert_eq!(nums, vec![1]);
    }
}

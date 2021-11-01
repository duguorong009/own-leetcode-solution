fn main() {
    println!("Hello, world!");
}

fn get_maximum_generate(n: i32) -> i32 {
    let mut nums: Vec<i32> = vec![0, 1];
    if n < 2 {
        return nums[n as usize];
    }
    for i in 2..=n {
        if i % 2 == 0 {
            nums.push(nums[i as usize / 2]);
        } else {
            let t = (i as usize - 1) / 2;
            nums.push(nums[t] + nums[t + 1]);
        }
    }
    nums.into_iter().max_by(|a, b| a.cmp(b)).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::get_maximum_generate;

    #[test]
    fn test_1() {
        let n = 7;
        assert_eq!(get_maximum_generate(n), 3);
    }

    #[test]
    fn test_2() {
        let n = 2;
        assert_eq!(get_maximum_generate(n), 1);
    }

    #[test]
    fn test_3() {
        let n = 3;
        assert_eq!(get_maximum_generate(n), 2);
    }

    #[test]
    fn test_4() {
        let n = 0;
        assert_eq!(get_maximum_generate(n), 0);
    }
}

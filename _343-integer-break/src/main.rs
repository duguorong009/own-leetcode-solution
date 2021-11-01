fn main() {
    println!("Hello, world!");
}

/*
    Problem: Given an integer n, break it into the sum of k POSITIVE INTEGERS,
    where k >= 2, and maximize the product of those integers.
    Return the maximum product you can get.

    Constraints: 2 <= n <= 58.

    Solution: Using the math knowledge that 2 and 3 are only numbers that could be
    broken into.
*/

fn integer_break(n: i32) -> i32 {
    if n <= 2 {
        return 1;
    }
    if n <= 3 {
        return 2;
    }

    let mut num_threes = n / 3;
    let mut num_twos = 0;

    if n % 3 == 1 {
        num_threes -= 1;
        num_twos = 2;
    } else if n % 3 == 2 {
        num_twos = 1;
    }

    2_i32.pow(num_twos) * 3_i32.pow(num_threes as u32)
}

#[cfg(test)]
mod tests {
    use crate::integer_break;

    #[test]
    fn test_1() {
        let n = 2;
        assert_eq!(integer_break(n), 1);
    }

    #[test]
    fn test_2() {
        let n = 10;
        assert_eq!(integer_break(n), 36);
    }
}

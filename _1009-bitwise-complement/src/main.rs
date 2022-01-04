fn main() {
    println!("Hello, world!");
}

fn bitwise_complement(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        // Get the power of 2, which is the least close to n.
        let mut two_power = 1;
        while two_power <= n {
            two_power *= 2;
        }
        two_power - n - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(bitwise_complement(5), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(bitwise_complement(7), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(bitwise_complement(10), 5);
    }

    #[test]
    fn test_4() {
        assert_eq!(bitwise_complement(16), 15);
    }
}

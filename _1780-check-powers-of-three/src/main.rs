fn main() {
    println!("Hello, world!");
}

fn check_powers_of_three(n: i32) -> bool {
    let mut n = n;
    while n != 0 {
        if n % 3 == 2 {
            return false;
        }
        n /= 3
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(check_powers_of_three(12));
        assert!(check_powers_of_three(91));
        assert!(!check_powers_of_three(21));
    }
}

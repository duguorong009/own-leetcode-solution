fn main() {
    println!("Hello, world!");
}

fn is_ugly(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    
    let mut n = n;

    while n % 2 == 0 {
        n /= 2;
    }
    while n % 3 == 0 { 
        n /= 3;
    }
    while n % 5 == 0 {
        n /= 5;
    }

    n == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ugly() {
        assert!(is_ugly(6));
        assert!(is_ugly(1));
        assert!(!is_ugly(14));
    }
}
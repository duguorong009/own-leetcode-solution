fn main() {
    println!("Hello, world!");
}

// // Ordinary solution
// fn is_power_of_three(n: i32) -> bool {
//     if n == 0 {
//         return false;
//     }

//     let mut n = n;
//     while n % 3 == 0 {
//         n /= 3;
//     }
//     n == 1
// }


// Challenge: No loops/recursions.
//
// Solution: The logic is very simple. 
// Any integer number other than power of 3 which divides highest power of 3 value
// that integer can hold 3^19 = 1162261467 (Assuming that integers are stored 
// using 32 bits) will give reminder non-zero. 
fn is_power_of_three(n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    /* The maximum power of 3 value that
       integer can hold is 1162261467 ( 3^19 ) .*/
    1162261467 % n == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_three() {
        assert!(is_power_of_three(27));
        assert!(!is_power_of_three(0));
        assert!(is_power_of_three(9));
    }
}
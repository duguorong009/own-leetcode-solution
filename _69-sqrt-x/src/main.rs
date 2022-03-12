fn main() {
    println!("Hello, world!");
}

// Super simple solution: (x as f64).sqrt().floor() as i32

fn my_sqrt(x: i32) -> i32 {
    let x = x as i64;
    let mut start: i64 = 0;
    let mut end: i64 = x;
    
    while start + 1 < end {
        let mid  = start + (end - start) / 2;
        if mid * mid == x {
            return mid as i32;
        } else if mid *  mid < x {
            start = mid;
        } else {
            end = mid;
        }
    }

    if end * end == x {
        return end as i32;
    }
    
    start as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let x = 4;
        assert_eq!(my_sqrt(x), 2);
    }

    #[test]
    fn test_2() {
        let x = 8;
        assert_eq!(my_sqrt(x), 2);
    }

    #[test]
    fn test_3() {
        let x = 2147483647;
        assert_eq!(my_sqrt(x), 46340);
    }
}
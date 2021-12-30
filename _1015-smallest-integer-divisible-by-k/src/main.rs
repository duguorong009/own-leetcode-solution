fn main() {
    println!("Hello, world!");
}

fn smallest_repunit_div_by_k(k: i32) -> i32 {
    println!("K::{}", k);
    let mut x: i32 = 0;
    for i in 0..k {
        x *= 10;
        x += 1;
        x %= k;
        if x % k == 0 {
            return i + 1;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::smallest_repunit_div_by_k;

    #[test]
    fn test_1() {
        let k = 1;
        assert_eq!(smallest_repunit_div_by_k(k), 1);
    }

    #[test]
    fn test_2() {
        let k = 2;
        assert_eq!(smallest_repunit_div_by_k(k), -1);
    }

    #[test]
    fn test_3() {
        let k = 3;
        assert_eq!(smallest_repunit_div_by_k(k), 3);
    }
}

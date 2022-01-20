fn main() {
    println!("Hello, world!");
}

fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut l = 1;
    let mut r = 1_000_000_000;
    while l < r {
        let m = (l + r) / 2;
        let mut sum = 0;
        for p in &piles {
            sum += if p % m == 0 { p / m } else { p / m + 1 };
        }
        if sum > h {
            l = m + 1;
        } else {
             r = m;
        }
    }
    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let piles = vec![3,6 ,7, 11];
        let h = 8;
        assert_eq!(min_eating_speed(piles, h), 4);
    }

    #[test]
    fn test_2() {
        let piles = vec![30 ,11 ,23, 4, 20];
        let h = 5;
        assert_eq!(min_eating_speed(piles, h), 30);
    }

    #[test]
    fn test_3() {
        let piles = vec![30 ,11 ,23, 4, 20];
        let h = 6;
        assert_eq!(min_eating_speed(piles, h), 23);
    }
}
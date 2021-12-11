fn main() {
    println!("Hello, world! ");
}

fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
    let mut low = 1;
    let mut high = 1e17 as i32;
    let lcm_a_b = lcm(&a, &b);
    while low < high {
        let mid = low + (high - low) / 2;
        let target = (mid / a) + (mid / b) - (mid / lcm_a_b);
        if target < n {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    high % 1_000_000_007
}

fn gcd(a: &i32, b: &i32) -> i32 {
    let a = *a;
    let b = *b;
    if b == 0 {
        return a;
    }
    gcd(&b, &(a % b))
}

fn lcm(a: &i32, b: &i32) -> i32 {
    (*a) * (*b) / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(nth_magical_number(1, 2, 3), 2);
    }
    #[test]
    fn test_2() {
        assert_eq!(nth_magical_number(4, 2, 3), 6);
    }
    #[test]
    fn test_3() {
        assert_eq!(nth_magical_number(5, 2, 4), 10);
    }

    #[test]
    fn test_4() {
        assert_eq!(nth_magical_number(3, 6, 4), 8);
    }
}

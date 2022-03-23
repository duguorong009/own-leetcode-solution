
fn main() {
    println!("Hello, world!");
}

fn broken_calc(start_value: i32, target: i32) -> i32 {
    let mut res = 0;
    let mut target = target;
    while target > start_value {
        if target % 2 == 0 {
            target /= 2;
        } else {
            target += 1;
        }
        res += 1;
    }
    res + start_value - target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(broken_calc(2, 3), 2);
        assert_eq!(broken_calc(5, 8), 2);
        assert_eq!(broken_calc(3, 10), 3);
    }

    #[test]
    fn test_exception() {
        assert_eq!(broken_calc(1024, 1), 1023);
    }
}
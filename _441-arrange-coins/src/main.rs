fn main() {
    println!("Hello, world!");
}

fn arrange_coins(n: i32) -> i32 {
    let mut n = n;
    if n <= 0 {
        return 0;
    }

    let mut count = 1;
    while n > 0 {
        n = n - count;
        if n < 0 {
            break;
        }
        count += 1;
    }
    count - 1
}

#[cfg(test)]
mod tests {
    use crate::arrange_coins;

    #[test]
    fn test_1() {
        let n = 5;
        assert_eq!(arrange_coins(n), 2);
    }

    #[test]
    fn test_2() {
        let n = 8;
        assert_eq!(arrange_coins(n), 3);
    }
}

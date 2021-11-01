fn main() {
    let a = 1;
    let b = 2;
    assert_eq!(get_sum(a, b), 3);
}

fn get_sum(a: i32, b: i32) -> i32 {
    println!("{}, {}", a, b);
    if b == 0 {
        a
    } else {
        get_sum(a ^ b, (a & b) << 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::get_sum;

    #[test]
    fn test_1() {
        let a = 1;
        let b = 2;
        assert_eq!(get_sum(a, b), 3);
    }

    #[test]
    fn test_2() {
        let a = 2;
        let b = 3;
        assert_eq!(get_sum(a, b), 5);
    }
}

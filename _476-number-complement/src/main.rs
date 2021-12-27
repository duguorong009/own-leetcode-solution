fn main() {
    find_complement(5);
}

fn find_complement(num: i32) -> i32 {
    let mut mask = !0;
    while mask & num != 0 {
        println!("{}, {}", mask, num);
        mask <<= 1;
    }
    !mask & !num
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(find_complement(5), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(find_complement(1), 0);
    }
}

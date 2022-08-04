fn main() {
    println!("Hello, world!");
}

fn mirror_reflection(p: i32, q: i32) -> i32 {
    let mut p = p;
    let mut q = q;
    while p % 2 == 0 && q % 2 == 0 {
        p >>= 1;
        q >>= 1;
    }
    1 - p % 2 + q % 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(mirror_reflection(2, 1), 2);
        assert_eq!(mirror_reflection(3, 1), 1);
    }
}

fn main() {
    println!("Hello, world!");
}

fn min_deletion_size(strs: Vec<String>) -> i32 {
    let mut d = 0;
    let a: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();
    let n = a.len();
    let m = a[0].len();
    for i in 0..m {
        for j in 1..n {
            if a[j][i] < a[j - 1][i] {
                d += 1;
                break;
            }
        }
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let strs = vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()];
        assert_eq!(min_deletion_size(strs), 1);
    }

    #[test]
    fn test_2() {
        let strs = vec!["a".to_string(), "b".to_string()];
        assert_eq!(min_deletion_size(strs), 0);
    }

    #[test]
    fn test_3() {
        let strs = vec!["zyx".to_string(), "wvu".to_string(), "tsr".to_string()];
        assert_eq!(min_deletion_size(strs), 3);
    }
}

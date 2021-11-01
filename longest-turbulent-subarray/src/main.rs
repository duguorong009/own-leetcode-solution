use std::cmp::Ordering::*;
fn main() {
    let arr: Vec<i32> = vec![9, 4, 2, 10, 7, 8, 8, 1, 9];
    println!("{}", max_turbulence_size(arr));
}

fn max_turbulence_size(arr: Vec<i32>) -> i32 {
    let a = arr;
    let n = a.len();
    let mut res = 1;
    let mut inc = 1;
    let mut dec = 1;
    for i in 1..n {
        match (a[i] - a[i - 1]).cmp(&0) {
            Equal => {
                inc = 1;
                dec = 1;
            }
            Less => {
                inc = dec + 1;
                dec = 1;
            }
            Greater => {
                dec = inc + 1;
                inc = 1;
            }
        }
        res = res.max(inc.max(dec));
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::max_turbulence_size;

    #[test]
    fn test_1() {
        let arr: Vec<i32> = vec![9, 4, 2, 10, 7, 8, 8, 1, 9];
        assert_eq!(max_turbulence_size(arr), 5);
    }

    #[test]
    fn test_2() {
        let arr: Vec<i32> = vec![4, 8, 12, 16];
        assert_eq!(max_turbulence_size(arr), 2);
    }

    #[test]
    fn test_3() {
        let arr: Vec<i32> = vec![100];
        assert_eq!(max_turbulence_size(arr), 1);
    }

    #[test]
    fn test_4() {
        let arr: Vec<i32> = vec![9, 9];
        assert_eq!(max_turbulence_size(arr), 1);
    }

    #[test]
    fn test_5() {
        let arr: Vec<i32> = vec![0, 1, 1, 0, 1, 0, 1, 1, 0, 0];
        assert_eq!(max_turbulence_size(arr), 5);
    }

    #[test]
    fn test_6() {
        let arr: Vec<i32> = vec![0, 8, 45, 88, 48, 68, 28, 55, 17, 24];
        assert_eq!(max_turbulence_size(arr), 8);
    }
}

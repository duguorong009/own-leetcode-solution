fn main() {
    println!("Hello, world!");
}

fn kth_factor(n: i32, k: i32) -> i32 {
    let mut factors: Vec<i32> = vec![];
    
    // Get all of factors
    let sqrt_n = (n as f32).sqrt() as i32;
    for i in 1..=sqrt_n {
        if n % i == 0 {
            if n / i == i {
                factors.push(i);
            } else {
                factors.push(i);
                factors.push(n / i);
            }
        }
    }

    // Calc the result
    if k > factors.len() as i32 {
        -1
    } else {
        factors.sort_unstable();
        factors[k as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(kth_factor(12, 3), 3);
        assert_eq!(kth_factor(7, 2), 7);
        assert_eq!(kth_factor(4, 4), -1);
    }
}
fn main() {
    println!("Hello, world!");
}

fn get_smallest_string(n: i32, k: i32) -> String {
    let mut k = k - n;
    let n = n as usize;
    let mut arr: Vec<u8> = vec![b'a'; n];
    for i in (0..n).rev() {
        if k > 25 {
            k -= 25;
            arr[i] = b'z';
        } else if k > 0 {
            arr[i] += k as u8;
            k -= k;
        } else {
            break;
        }
    }
    arr.into_iter().map(|b| b as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(get_smallest_string(3, 27), "aay".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(get_smallest_string(5, 73), "aaszz".to_string());
    }
}
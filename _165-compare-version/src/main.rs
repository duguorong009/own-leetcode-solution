fn main() {
    println!("Hello, world!");
}

fn compare_version(version1: String, version2: String) -> i32 {
    
    let v1: Vec<i32> = version1
        .split(".")
        .map(|v| v.parse::<i32>().unwrap_or(i32::MIN))
        .collect();
    let v2: Vec<i32> = version2
        .split(".")
        .map(|v| v.parse::<i32>().unwrap_or(i32::MIN))
        .collect();

    let n = v1.len().max(v2.len());

    for i in 0..n {
        let v1_rev = if i < v1.len() { v1[i] } else { 0 };
        let v2_rev = if i < v2.len() { v2[i] } else { 0 };
        if v1_rev < v2_rev {
            return -1;
        } 
        if v1_rev > v2_rev {
            return 1;
        }
    }
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let version1 = "1.01".to_string();
        let version2 = "1.001".to_string();
        assert_eq!(compare_version(version1, version2), 0);
    }

    #[test]
    fn test_2() {
        let version1 = "1.0".to_string();
        let version2 = "1.0.0".to_string();
        assert_eq!(compare_version(version1, version2), 0);
    }

    #[test]
    fn test_3() {
        let version1 = "0.1".to_string();
        let version2 = "1.1".to_string();
        assert_eq!(compare_version(version1, version2), -1);
    }
}
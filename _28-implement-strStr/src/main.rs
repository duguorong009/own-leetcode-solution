fn main() {
    println!("Hello, world!");
}

// // What should we return when needle is an empty string? This is a great question to ask during an interview.
// // For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's strstr() and Java's indexOf().

// Super simple solution using built-in function
fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(v) => v as i32,
        None => -1,
    }
}

// // Own solution: Time-Limit Exceeded error
// fn str_str(haystack: String, needle: String) -> i32 {
//     if needle.is_empty() {
//         return 0;
//     }
    
//     let haystack: Vec<char> = haystack.chars().collect();
//     let needle: Vec<char> = needle.chars().collect();
//     let n = haystack.len();
//     let m = needle.len();

//     let mut i = 0;
//     while i < n {
//         let mut j = 0;
//         if haystack[i] == needle[j] {
//             while j < m && i + j < n {
//                 println!("{}:{}", i, j);
//                 if haystack[i + j] != needle[j] {
//                     break;
//                 }
//                 j += 1;
//             }
//             if j == m {
//                 return i as i32;
//             }
//         } 
//         i += 1;
//     }
//     -1
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let haystack = "hello".to_string();
        let needle = "ll".to_string();
        assert_eq!(str_str(haystack, needle), 2);
    }

    #[test]
    fn test_2() {
        let haystack = "aaaa".to_string();
        let needle = "bba".to_string();
        assert_eq!(str_str(haystack, needle), -1);
    }

    #[test]
    fn test_3() {
        let haystack = "".to_string();
        let needle = "".to_string();
        assert_eq!(str_str(haystack, needle), 0);
    }
}
fn main() {
    println!("{}", reverse_words("Hello, world!".to_string()));
}
// // My own solution.
// fn reverse_words(s: String) -> String {
//     let words: Vec<String> = s.split(" ").map(|s| s.to_string()).collect();
//     let mut reversed_words: Vec<String> = vec![];
//     for word in words {
//         reversed_words.push(reverse_single_word(word));
//     }
//     reversed_words.join(" ").to_string()
// }

// fn reverse_single_word(s: String) -> String {
//     if s.len() == 1 {
//         s
//     } else {
//         let mut chars: Vec<&str> = s.split("").collect();
//         println!("{:?}, {}", chars, chars.len());
//         let mut l = 0;
//         let mut r = chars.len() - 1;
//         while l <= r {
//             let temp = chars[l];
//             chars[l] = chars[r];
//             chars[r] = temp;
//             l += 1;
//             r -= 1;
//         }
//         chars.join("").to_string()
//     }
// }

// // Rust way
fn reverse_words(s: String) -> String {
    let words: Vec<String> = s
        .split_whitespace()
        .map(|s| s.chars().rev().collect())
        .collect();
    let res: String = words.join(" ");
    res
}

#[cfg(test)]
mod tests {
    use crate::reverse_words;

    #[test]
    fn test_1() {
        let s = "Let's take LeetCode contest".to_string();
        assert_eq!(reverse_words(s), "s'teL ekat edoCteeL tsetnoc".to_string());
    }

    #[test]
    fn test_2() {
        let s = "God Ding".to_string();
        assert_eq!(reverse_words(s), "doG gniD".to_string());
    }
}

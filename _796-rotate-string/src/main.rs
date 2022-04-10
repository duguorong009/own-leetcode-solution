fn main() {
    println!("Hello, world!");
}

// fn rotate_string(s: String, goal: String) -> bool {
//     let s: Vec<char> = s.chars().collect();
//     let mut goal: Vec<char> = goal.chars().collect();

//     let n = goal.len();
//     for _ in 0..n {
//         goal.rotate_left(1);
//         if s == goal {
//             return true;
//         }
//     }

//     false
// }

fn rotate_string(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }

    let mut double_s = "".to_string();
    double_s.push_str(s.as_str());
    double_s.push_str(s.as_str());

    double_s.contains(goal.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "abcde".to_string();
        let goal = "cdeab".to_string();
        assert!(rotate_string(s, goal));
    }

    #[test]
    fn test_2() {
        let s = "abcde".to_string();
        let goal = "abced".to_string();
        assert!(!rotate_string(s, goal));
    }
}

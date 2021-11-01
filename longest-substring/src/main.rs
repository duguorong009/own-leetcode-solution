use std::collections::HashMap;
fn length_of_longest_substring(s: String) -> i32 {
    let mut hm: HashMap<char, usize> = HashMap::new();
    let mut max: usize = 0;
    let mut l: usize = 0;
    for (r, c) in s.char_indices() {
        if let Some(end) = hm.insert(c, r) {
            l = usize::max(l, end + 1);
        }
        max = usize::max(r - l + 1, max);
    }
    max as i32
}

fn main() {
    let s = String::from("abcabcbb");
    println!("{}", length_of_longest_substring(s));
}

#[cfg(test)]
mod tests {
    use crate::length_of_longest_substring;

    #[test]
    fn work_1() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }

    #[test]
    fn work_2() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn work_3() {
        assert_eq!(length_of_longest_substring("bbbbbb".to_string()), 1);
    }

    #[test]
    fn work_4() {
        assert_eq!(length_of_longest_substring("abcdeabcdef".to_string()), 6);
    }

    #[test]
    fn work_5() {
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }

    #[test]
    fn work_6() {
        assert_eq!(
            length_of_longest_substring(
                "z823 -0*&#)(@+_)@#  AFlje.mboia 898hgf984l@$TG#$g".to_string()
            ),
            15
        );
    }

    #[test]
    fn work_7() {
        assert_eq!(length_of_longest_substring("cdd".to_string()), 2);
    }

    #[test]
    fn work_8() {
        assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
    }

    #[test]
    fn work_9() {
        assert_eq!(length_of_longest_substring("asjrgapa".to_string()), 6);
    }
}

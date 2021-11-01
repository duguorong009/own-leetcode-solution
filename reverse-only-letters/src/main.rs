fn main() {
    let s = "ab-cd".to_string();
    assert_eq!(reverse_only_letters(s), "dc-ba".to_string());
}

// Apply two-pointer approach.
// So, it sets left and right pointer as first and last index of array.
// After that, the loop starts and
// move left pointer to right direction &
// move right pointer to left direction.
// When loop, it adds logic.
// So, if the pointer data(s[l], s[r]) is not alphabetic,
// it moves the pointer(increase/decrease the index).
// If the pointer data(s[l], s[r]) is alphabetic,
// then performs swap operation.
fn reverse_only_letters(s: String) -> String {
    let mut chars_array: Vec<char> = s.chars().collect();
    let mut left: usize = 0;
    let mut right: usize = chars_array.len() - 1;
    loop {
        if left >= right {
            break;
        }
        if !chars_array[left].is_ascii_alphabetic() && left < right {
            left += 1;
        } else if !chars_array[right].is_ascii_alphabetic() && left < right {
            right -= 1;
        } else {
            let temp = chars_array[right];
            chars_array[right] = chars_array[left];
            chars_array[left] = temp;
            left += 1;
            right -= 1;
        }
    }
    chars_array.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::reverse_only_letters;

    #[test]
    fn test_1() {
        let s = "ab-cd".to_string();
        assert_eq!(reverse_only_letters(s), "dc-ba".to_string());
    }

    #[test]
    fn test_2() {
        let s = "a-bC-dEf-ghIj".to_string();
        assert_eq!(reverse_only_letters(s), "j-Ih-gfE-dCba".to_string());
    }

    #[test]
    fn test_3() {
        let s = "Test1ng-Leet=code-Q!".to_string();
        assert_eq!(reverse_only_letters(s), "Qedo1ct-eeLg=ntse-T!".to_string());
    }
}


pub fn longest_valid_parentheses(s: String) -> i32 {

    // Convert the "s" to arr of "char".
    let s: Vec<char> = s.chars().collect();

    // Select the slice(range) of "s" arr.(from up to bottom)
    let mut start = 0;
    let mut end = s.len();
    while start < end {
        if s[start] == ')' {
            start += 1;
            continue;
        }

        if s[end] == '(' {
            end -= 1;
            continue;
        }
        let selected = s[start..end].to_vec();

        // Check if the slice is "valid" parentheses.
        let is_valid = is_valid_parentheses(&selected);

        // Return the length of first valid parentheses
        if is_valid {
            return selected.len() as i32;
        }
    }
    0
}

fn is_valid_parentheses(s: &Vec<char>) -> bool {

    let mut stack: Vec<char> = vec![];
    for &ch in s {
        match ch {
            '(' => { stack.push('('); },
            ')' => { 
                if stack.pop().is_none() {
                    return false;
                }
            },
            _ => panic!("Do not enter here!"),
        }
    }
    if stack.is_empty() {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_paren() {
        assert_eq!(is_valid_parentheses(&vec!['(', ')']), true);
    }
}
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn frequency_sorts(s: String) -> String {
    let mut freq_table: HashMap<&str, i32> = HashMap::new();
    let chars = s.split("").collect::<Vec<&str>>();
    for &char in chars.iter() {
        *(freq_table.entry(char).or_insert(0)) += 1;
    }

    let mut chars: Vec<(&str, i32)> = vec![];
    for (char, cnt) in freq_table {
        chars.push((char, cnt));
    }
    chars.sort_by_key(|(_, cnt)| *cnt);
    let chars = chars
        .iter()
        .rev()
        .map(|(char, cnt)| vec![*char; *cnt as usize].concat())
        .collect::<Vec<String>>();
    chars.join("")
}

#[cfg(test)]
mod tests {
    use crate::frequency_sorts;

    #[test]
    fn test_1() {
        let s = "tree".to_string();
        assert_eq!(frequency_sorts(s), "eert".to_string());
    }

    #[test]
    fn test_2() {
        let s = "cccaaa".to_string();
        assert_eq!(frequency_sorts(s), "aaaccc".to_string());
    }

    #[test]
    fn test_3() {
        let s = "Aabb".to_string();
        assert_eq!(frequency_sorts(s), "bbaA".to_string());
    }
}

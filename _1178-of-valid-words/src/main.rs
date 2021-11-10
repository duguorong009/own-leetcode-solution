use std::collections::HashMap;

fn main() {
    let words = vec![
        "aaaa".to_string(),
        "asas".to_string(),
        "able".to_string(),
        "ability".to_string(),
        "actt".to_string(),
        "actor".to_string(),
        "access".to_string(),
    ];
    let puzzles = vec![
        "aboveyz".to_string(),
        "abrodyz".to_string(),
        "abslute".to_string(),
        "abosoryz".to_string(),
        "actresz".to_string(),
        "gaswxyz".to_string(),
    ];
    assert_eq!(
        find_num_of_valid_words(words, puzzles),
        vec![1, 1, 3, 2, 4, 0]
    );
}

fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    // kind of like the anagram type of hashing? 26-tuple hashing?
    // - first put all the words into a map (by doing that 26-tuple hashing)
    // - then loop thru each word in puzzles and generate all subsets
    //
    // for puzzle in puzzles:
    //     generate all subsets of this word, must include first letter?. check if that subset is in the map

    let mut map: HashMap<i32, i32> = HashMap::new();
    for w in words {
        let mut mask = 0;
        for c in w.chars() {
            mask |= 1 << c as u32 - 'a' as u32;
        }
        *(map.entry(mask).or_insert(0)) += 1;
    }

    for p in puzzles {
        let mut valid = 0;
        let n = p.chars().into_iter().count() - 1;
        for i in 0..1 << n {
            let mut mask = 1 << p.chars().nth(0).unwrap() as u32 - 'a' as u32;
            for j in 0..n {
                if (i & (1 << j)) > 0 {
                    mask |= 1 << p.chars().nth(j + 1).unwrap() as u32 - 'a' as u32;
                }
            }
            if map.contains_key(&mask) {
                valid += map.get(&mask).unwrap();
            }
        }
        res.push(valid);
    }
    res
}

fn gen_all_subsets(id: usize, mask: i32) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let words = vec![
            "aaaa".to_string(),
            "asas".to_string(),
            "able".to_string(),
            "ability".to_string(),
            "actt".to_string(),
            "actor".to_string(),
            "access".to_string(),
        ];
        let puzzles = vec![
            "aboveyz".to_string(),
            "abrodyz".to_string(),
            "abslute".to_string(),
            "abosoryz".to_string(),
            "actresz".to_string(),
            "gaswxyz".to_string(),
        ];
        assert_eq!(
            find_num_of_valid_words(words, puzzles),
            vec![1, 1, 3, 2, 4, 0]
        );
    }

    #[test]
    fn test_2() {
        let words = vec![
            "apple".to_string(),
            "pleas".to_string(),
            "please".to_string(),
        ];
        let puzzles = vec![
            "aelwxyz".to_string(),
            "aelpxyz".to_string(),
            "aelpxsy".to_string(),
            "saelpxy".to_string(),
            "xaelpsy".to_string(),
        ];
        assert_eq!(find_num_of_valid_words(words, puzzles), vec![0, 1, 3, 2, 0]);
    }
}

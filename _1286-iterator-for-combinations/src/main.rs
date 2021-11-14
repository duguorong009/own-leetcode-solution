fn main() {
    println!("Hello, world!");
    let com_iter = CombinationIterator::new("abcd".to_string(), 3);
}

struct CombinationIterator {
    combinations: Vec<String>,
    index: usize,
}

impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        let n = combination_length as usize;
        let m = characters.len();
        let mut indexes = vec![];
        let mut combinations = vec![];
        let s: Vec<char> = characters.chars().collect();
        Self::dfs(0, &mut indexes, &mut combinations, &s, n, m);
        let index = 0;
        CombinationIterator {
            combinations,
            index,
        }
    }

    fn next(&mut self) -> String {
        let res = self.combinations[self.index].to_string();
        self.index += 1;
        res
    }

    fn has_next(&self) -> bool {
        self.index < self.combinations.len()
    }

    fn dfs(
        start: usize,
        indexes: &mut Vec<usize>,
        combinations: &mut Vec<String>,
        s: &[char],
        n: usize,
        m: usize,
    ) {
        if indexes.len() == n {
            let t: String = indexes.iter().map(|&i| s[i]).collect();
            combinations.push(t);
        } else {
            for i in start..m {
                indexes.push(i);
                Self::dfs(i + 1, indexes, combinations, s, n, m);
                indexes.pop();
            }
        }
    }
}

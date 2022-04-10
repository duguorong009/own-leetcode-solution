fn main() {
    println!("Hello, world!");
}


struct UnionFind {
    parent: Vec<usize>,
    n: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        UnionFind { parent, n }
    }

    fn find(&mut self, i: usize) -> usize {
        let j = self.parent[i];
        if i == j {
            j
        } else {
            let k = self.find(j);
            self.parent[i] = k;
            k
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i != j {
            self.parent[i] = j;
            self.n -= 1;
        }
    }
}

fn equations_possible(equations: Vec<String>) -> bool {
    let mut uf = UnionFind::new(26);
    let mut pairs: Vec<(usize, usize)> = vec![];
    for equation in equations {
        let s: Vec<char> = equation.chars().collect();
        let i = (s[0] as u8 - b'a') as usize;
        let j = (s[3] as u8 - b'a') as usize;
        if s[1] == '=' {
            uf.union(i, j);
        } else {
            pairs.push((i, j));
        }
    }
    for pair in pairs {
        let i = pair.0;
        let j = pair.1;
        if uf.find(i) == uf.find(j) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let equations = vec!["a==b".to_string(), "b!=a".to_string()];
        assert!(!equations_possible(equations));
    }

    #[test]
    fn test_2() {
        let equations = vec!["a==b".to_string(), "b==a".to_string()];
        assert!(equations_possible(equations));
    }

    #[test]
    fn test_3() {
        let equations = vec!["a==b".to_string(),"b!=c".to_string(), "c==a".to_string()];
        assert!(!equations_possible(equations));
    }
}
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn dest_city(paths: Vec<Vec<String>>) -> String {
    let n = paths.len();
    let mut hs: HashSet<String> = HashSet::new();
    paths.iter().for_each(|v| { hs.insert(v[0].clone()); } );
    for i in 0..n {
        if !hs.contains(&paths[i][1]) {
            return paths[i][1].clone();
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let paths = vec![vec!["London".to_string(), "New York".to_string()], vec!["New York".to_string(), "Lima".to_string()], vec!["Lima".to_string(), "Sao Paulo".to_string()]]; 
        assert_eq!(dest_city(paths), "Sao Paulo".to_string());
    }

    #[test]
    fn test_2() {
        let paths = vec![vec!["B".to_string(), "C".to_string()], vec!["D".to_string(), "B".to_string()], vec!["C".to_string(), "A".to_string()]];
        assert_eq!(dest_city(paths), "A".to_string());
    }

    #[test]
    fn test_3() {
        let paths = vec![vec!["A".to_string(), "Z".to_string()]];
        assert_eq!(dest_city(paths), "Z".to_string());
    }
}
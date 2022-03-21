
fn main() {
    println!("Hello, world!");
}

fn partition_labels(s: String) -> Vec<i32> {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();

    // Create the ranges of every character.
    let mut i = 0;
    let mut j = n - 1;
    let mut appeared_chs: Vec<i32> = vec![0; 26];
    let mut ranges: Vec<(usize, usize)> = vec![];
    while i < n {
        let ch = s[i];
        let id = (ch as u8 - b'a') as usize;
        if appeared_chs[id] == 0 {
            while s[j] != ch {
                j -= 1;
            }
            appeared_chs[(ch as u8 - b'a') as usize] = 1;
            ranges.push((i, j));
            j = n - 1;
        }
        i += 1;
    }

    // Remove the overlapped area.
    let n = ranges.len();
    if n == 1 {
        return vec![n as i32];
    }

    let mut res: Vec<(usize, usize)> = vec![ranges[0]];
    for i in 1..n {
        let (s1, e1) = res.pop().expect("Empty ranges");
        let (s2, e2) = ranges[i];
        if s2 < e1 {
            res.push((s1, usize::max(e1, e2)));
        } else {
            res.push((s1, e1));
            res.push((s2, e2));
        }
    }

    res.into_iter().map(|(s, e)| (e - s + 1) as i32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "ababcbacadefegdehijhklij".to_string();
        assert_eq!(partition_labels(s), vec![9, 7, 8]);
    }

    #[test]
    fn test_2() {
        let s = "eccbbbbdec".to_string();
        assert_eq!(partition_labels(s), vec![10]);
    }
}
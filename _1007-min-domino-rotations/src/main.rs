fn main() {
    println!("Hello, world!");
}

fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    let n = tops.len();
    let mut freq: Vec<i32> = vec![0; 7];
    for i in 0..n {
        freq[tops[i] as usize] += 1;
        freq[bottoms[i] as usize] += 1;
    }

    let mut target = 0;
    for i in 0..7 {
        if freq[i] >= n as i32 {
            target = i as i32;
        }
    }

    if target == 0 {
        return -1;
    } 

    let mut tc = 0;
    let mut bc = 0;
    for i in 0..n {
        if tops[i] != target && bottoms[i] != target {
            return -1;
        }
        if tops[i] != target {
            tc += 1;
        }
        if bottoms[i] != target {
            bc += 1;
        }
    }
    if tc < bc {
        tc
    } else {
        bc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let tops = vec![2, 1, 2, 4, 2, 2];
        let bottoms = vec![5, 2, 6, 2, 3, 2];
        assert_eq!(min_domino_rotations(tops, bottoms), 2);
    }

    #[test]
    fn test_2() {
        let tops = vec![3, 5, 1, 2, 3];
        let bottoms = vec![3, 6, 3, 3, 4];
        assert_eq!(min_domino_rotations(tops, bottoms), -1);
    }

    #[test]
    fn test_3() {
        let tops = vec![1, 2, 1, 1, 1, 2, 2, 2];
        let bottoms = vec![2, 1, 2, 2, 2, 2, 2, 2];
        assert_eq!(min_domino_rotations(tops, bottoms), 1);
    }
}
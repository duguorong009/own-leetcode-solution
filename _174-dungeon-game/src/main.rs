fn main() {
    let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
    assert_eq!(calculate_minimum_hp(dungeon), 7);
}

fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
    let m = dungeon.len();
    let n = dungeon[0].len();
    if m == 1 && n == 1 {
        return dungeon[0][0] + 1;
    }

    let mut hp: Vec<Vec<i32>> = vec![vec![0; n]; m];

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i + 1 < m && j + 1 < n {
                hp[i][j] = 1.max(-dungeon[i][j] + hp[i + 1][j].min(hp[i][j + 1]));
                continue;
            }
            if i + 1 < m {
                hp[i][j] = 1.max(-dungeon[i][j] + hp[i + 1][j]);
                continue;
            }
            if j + 1 < n {
                hp[i][j] = 1.max(-dungeon[i][j] + hp[i][j + 1]);
                continue;
            }
            hp[i][j] = 1.max(-dungeon[i][j] + 1);
        }
    }

    hp[0][0]
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::calculate_minimum_hp;

    #[test]
    fn test_1() {
        let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
        assert_eq!(calculate_minimum_hp(dungeon), 7);
    }

    #[test]
    fn test_2() {
        let dungeon = vec![vec![0]];
        assert_eq!(calculate_minimum_hp(dungeon), 1);
    }
}

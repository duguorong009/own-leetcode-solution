fn main() {
    println!("Hello, world!");
}

const DIVIDER: usize = 1_000_000_007;

fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
    let mut count = vec![0; 101];
        for x in arr {
            count[x as usize] += 1;
        }
        let mut res = 0;
        for x in 0..101 {
            for y in x + 1..101 {
                if x + y >= target as usize {
                    break;
                }
                for z in y + 1..101 {
                    if x + y + z > target as usize {
                        break;
                    }
                    if x + y + z == target as usize {
                        res += count[x] * count[y] * count[z];
                        res %= DIVIDER;
                    }
                }
            }
        }
        for x in 0..101 {
            for y in x + 1..101 {
                if x + x + y != target as usize {
                    continue;
                }
                if count[x] > 1 {
                    res += count[x] * (count[x] - 1) / 2 * count[y];
                    res %= DIVIDER;
                }
            }
        }
        for x in 0..101 {
            for y in x + 1..101 {
                if x + y + y != target as usize {
                    continue;
                }
                if count[y] > 1 {
                    res += count[x] * count[y] * (count[y] - 1) / 2;
                    res %= DIVIDER;
                }
            }
        }
        for x in 0..101 {
            if x + x + x != target as usize {
                continue;
            }
            if count[x] > 2 {
                res += count[x] * (count[x] - 1) * (count[x] - 2) / 6;
                res %= DIVIDER;
            }
        }
        res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
        let target = 8;
        assert_eq!(three_sum_multi(arr, target), 20);
    }

    #[test]
    fn test_2() {
        let arr = vec![1, 1, 2, 2, 2, 2];
        let target = 5;
        assert_eq!(three_sum_multi(arr, target), 12);
    }
}
fn main() {
    println!("Hello, world!");
}

fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut flowerbed = flowerbed;
    let n = flowerbed.len();
    let mut sum = 0;
    for i in 0..n {
        if flowerbed[i] == 0 {
            if i == 0 {
                sum += 1;
                flowerbed[i] = 1;
            } else {
                if flowerbed[i - 1] == 0 {
                    sum += 1;
                    flowerbed[i] = 1;
                }
            }
        } else {
            if i > 0 && flowerbed[i - 1] == 1 {
                sum -= 1;
            }
        }
    }

    sum >= n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        assert_eq!(can_place_flowers(flowerbed, n), true);
    }

    #[test]
    fn test_2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        assert_eq!(can_place_flowers(flowerbed, n), false);
    }
}
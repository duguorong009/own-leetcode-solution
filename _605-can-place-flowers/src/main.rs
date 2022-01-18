fn main() {
    println!("Hello, world!");
}

fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut flowerbed = flowerbed;
    let mut n = n;

    // Insert the `0` at the head and tail of the `flowerbed`.
    flowerbed.insert(0, 0 );
    flowerbed.push(0);  

    let m = flowerbed.len();

    for i in 1..m - 1 {
        if flowerbed[i - 1] == 0 && flowerbed[i] == 0 && flowerbed[i + 1] == 0 {
            flowerbed[i] = 1;
            n -= 1;
        }
    }

    n <= 0
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
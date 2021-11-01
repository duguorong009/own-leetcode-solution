fn main() {
    let image: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    let sr = 1;
    let sc = 1;
    let new_color = 2;
    let expect: Vec<Vec<i32>> = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
    let result = flood_fill(image, sr, sc, new_color);
    assert_eq!(result, expect);
}

fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    let mut image = image;
    let m = image.len();
    let n = image[0].len();
    let sr = sr as usize;
    let sc = sc as usize;
    let origin_color = image[sr][sc];
    if origin_color == new_color {
        return image;
    }
    dfs(&mut image, sr, sc, m, n, origin_color, new_color);
    image
}

fn dfs(
    image: &mut Vec<Vec<i32>>,
    r: usize,
    c: usize,
    m: usize,
    n: usize,
    color: i32,
    new_color: i32,
) {
    if image[r][c] == color {
        image[r][c] = new_color;
        if r >= 1 {
            dfs(image, r - 1, c, m, n, color, new_color);
        }
        if r + 1 < m {
            dfs(image, r + 1, c, m, n, color, new_color);
        }
        if c >= 1 {
            dfs(image, r, c - 1, m, n, color, new_color)
        }
        if c + 1 < n {
            dfs(image, r, c + 1, m, n, color, new_color);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::flood_fill;

    #[test]
    fn test_1() {
        let image: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let sr = 1;
        let sc = 1;
        let new_color = 2;
        let expect: Vec<Vec<i32>> = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        let result = flood_fill(image, sr, sc, new_color);
        assert_eq!(result, expect);
    }

    #[test]
    fn test_2() {
        let image: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![0, 0, 0]];
        let sr = 0;
        let sc = 0;
        let new_color = 2;
        let expect: Vec<Vec<i32>> = vec![vec![2, 2, 2], vec![2, 2, 2]];
        let result = flood_fill(image, sr, sc, new_color);
        assert_eq!(result, expect);
    }
}

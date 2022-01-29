fn main() {
    let heights = vec![2, 1, 5, 6, 2, 3];
    assert_eq!(largest_rectangle_area(heights), 10);
}

fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut heights = heights;
    let mut stack: Vec<(i32, i32)> = vec![(0,0)];
    heights.push(0);
    let n = heights.len();
    let mut res = 0; 
    for i in 0..n {
        let x1 = (i + 1) as i32;
        let y1= heights[i];
        let mut x3 = x1;
        while let Some(&(x2, y2)) = stack.last() {
            println!("{:?}", stack);
            if y2 > y1 {
                stack.pop();
                res = res.max((x1 - x2) * y2);
                x3 = x2;
            } else {
                stack.push((x3, y1));
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(largest_rectangle_area(heights), 10);
    }

    #[test]
    fn test_2() {
        let heights = vec![2, 4];
        assert_eq!(largest_rectangle_area(heights), 4);
    }
}
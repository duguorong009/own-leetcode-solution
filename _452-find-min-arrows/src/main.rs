fn main() {
    println!("Hello, world!");
}

//// my own version - 46/48 tests passed.
// fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
//     // Todo
//     let mut points = points;
//     points.sort_unstable();
//     let mut n = points.len();
//     println!("{:?}", points);
//     // If only one balloon, need only 1 shot.
//     if n == 1 {
//         return 1;
//     }

//     let mut i = 0;
//     while i < n  {
//         if i <  n - 1 && points[i + 1][0] <= points[i][1] {
//             points[i][1] = points[i + 1][0].max(points[i][1]);
//             points[i][1] = points[i + 1][1].min(points[i][1]);
//             points.remove(i + 1);
//             n = points.len();
//         } else {
//             i += 1;
//         }
//     }
//     points.len() as i32 
// }

fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    let mut points = points;
    let n = points.len();
    if n == 0 {
        return 0;
    }
    points.sort_by_key(|p| p[1]);
    let mut end = points[0][1];
    let mut res = 1;
    for i in 1..n {
        if points[i][0] <= end {
            continue;
        }
        end = points[i][1];
        res += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        assert_eq!(find_min_arrow_shots(points), 2);
    }

    #[test]
    fn test_2() {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        assert_eq!(find_min_arrow_shots(points), 4);
    }

    #[test]
    fn test_3() {
        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        assert_eq!(find_min_arrow_shots(points), 2);
    }

    #[test]
    fn test_4() {
        let points = vec![vec![3,9],vec![7,12],vec![3,8],vec![6,8],vec![9,10],vec![2,9],vec![0,9],vec![3,9],vec![0,6],vec![2,8]];
        assert_eq!(find_min_arrow_shots(points), 2);
    }

    #[test]
    fn test_5() {
        let points = vec![vec![9,12],vec![1,10],vec![4,11],vec![8,12],vec![3,9],vec![6,9],vec![6,7]];
        assert_eq!(find_min_arrow_shots(points), 2);
    }
}
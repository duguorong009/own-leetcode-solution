fn main() {
    println!("Hello, world!");
}

// Use the binary search to get the optimized search result
pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = matrix.len();

    if k == 0 || k > (n * n) as i32 || n == 0 {
        return -1;
    }

    if k == 1 {
        return matrix[0][0];
    }

    // define smallest & largest element in the matrix as
    // lower range &
    // upper range
    let mut lo = matrix[0][0];
    let mut hi = matrix[n - 1][n - 1];

    // Perform binary search (by value)
    // between smallest(top-left) and largest(bottom-down) values
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let mut count = 0;
        let mut j = n - 1;

        // find out how many numbers are greater than mid
        // between lo and hi
        for i in 0..n {
            while j > 0 && matrix[i][j] > mid {
                j -= 1;
            }
            count += j + 1;
        }

        // if number of such element is less than k
        // narrow the search range by increasing lo value
        if count < k as usize {
            lo = mid + 1;
        } 
        // if number of such element is greater or equal to k
        // narrow the search range by decreasing hi value
        else {
            hi = mid;
        }
    }

    // after iteratively narrowing the search range
    // you narrow down to a single element in the matrix
    // which is k-th smallest element
    return lo;

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
        let k = 8;
        assert_eq!(kth_smallest(matrix, k), 13);
    }
    #[test]
    fn test_2() {
        let matrix = vec![vec![-5]];
        let k = 1;
        assert_eq!(kth_smallest(matrix, k), -5);
    }
}
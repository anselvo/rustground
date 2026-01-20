pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (mut ml, mut mr) = (0, matrix.len() - 1);
    let (mut nl, mut nr) = (0, matrix[0].len() - 1);
    while ml < mr {
        let m = (ml + mr) / 2;
        if matrix[m][nr] < target {
            ml = m + 1;
        } else {
            mr = m;
        }
    }
    while nl < nr {
        let m  = (nl + nr) / 2;
        if matrix[ml][m] == target { return true }
        if matrix[ml][m] < target {
            nl = m + 1;
        } else {
            nr = m;
        }
    }
    if matrix[ml][nl] == target { true } else { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 60), true);
        assert_eq!(search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 3), true);
        assert_eq!(search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 13), false);
        assert_eq!(search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 61), false);
        assert_eq!(search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 1), true);
        assert_eq!(search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 2), false);
        assert_eq!(search_matrix(vec![vec![1]], 1), true);
        assert_eq!(search_matrix(vec![vec![1]], 12), false);
    }
}

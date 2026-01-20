pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 { return vec![-1,-1] }
    let (mut ll, mut rl) = (0, nums.len() - 1);
    while rl - ll > 0 {
        let ml = (ll + rl) / 2;
        if nums[ml] < target {
            ll = ml + 1;
        } else {
            rl = ml;
        }
    }
    let (mut lr, mut rr) = (0, nums.len() - 1);
    while rr - lr > 0 {
        let mr = (lr + rr + 1) / 2;
        if nums[mr] > target {
            rr = mr - 1;
        } else {
            lr = mr;
        }
    }
    if ll > rr || nums[ll] != target || nums[rr] != target {
        vec![-1, -1]
    } else {
        vec![ll as i32, rr as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(search_range(vec![4, 4, 4, 4, 4, 7], 4), vec![0, 4]);
        assert_eq!(search_range(vec![1, 4, 4, 4, 4, 4], 4), vec![1, 5]);
        assert_eq!(search_range(vec![1, 1, 1, 1, 1, 1], 1), vec![0, 5]);
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
        assert_eq!(search_range(vec![], 0), vec![-1, -1]);
        assert_eq!(search_range(vec![1, 3, 5], 5), vec![2, 2]);
        assert_eq!(search_range(vec![1], 1), vec![0, 0]);
        assert_eq!(search_range(vec![0, 1, 2, 3], 0), vec![0, 0]);
        assert_eq!(search_range(vec![0, 1, 2, 3], 4), vec![-1, -1]);
        assert_eq!(search_range(vec![4, 5, 6, 7], 4), vec![0, 0]);
        assert_eq!(search_range(vec![4, 5, 6, 7], 3), vec![-1, -1]);
        assert_eq!(search_range(vec![1], 0), vec![-1, -1]);
        assert_eq!(search_range(vec![0, 1, 2, 4, 5, 6, 7], 2), vec![2, 2]);
    }
}

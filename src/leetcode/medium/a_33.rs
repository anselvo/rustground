pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    // NOTE: find rotation using binary search
    let (mut lr, mut rr) = (0, nums.len() - 1);
    while rr - lr > 0 {
        let mid = (lr + rr) / 2;
        if nums[mid] < nums[rr] { rr = mid; }
        else { lr = mid + 1; }
    }
    // NOTE: find target using binary search with possible rotation
    let (mut l, mut r) = (0, nums.len());
    while r - l > 0 {
        let mid = (l + r) / 2;
        let real_mid = (mid + lr) % nums.len();
        if nums[real_mid] == target { return real_mid as i32 }
        if nums[real_mid] < target {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(search(vec![4,5,6,7,0,1,2], 0), 4);
        assert_eq!(search(vec![5,1,3], 5), 0);
        assert_eq!(search(vec![1], 1), 0);
        assert_eq!(search(vec![0,1,2,3], 0), 0);
        assert_eq!(search(vec![0,1,2,3], 4), -1);
        assert_eq!(search(vec![4,5,6,7,0,1,2], 4), 0);
        assert_eq!(search(vec![4,5,6,7,0,1,2], 3), -1);
        assert_eq!(search(vec![1], 0), -1);
        assert_eq!(search(vec![4,5,6,7,0,1,2], 2), 6);
    }
}
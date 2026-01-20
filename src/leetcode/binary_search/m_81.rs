// NOTE: the same as a_33.rs but with duplications
pub fn search(nums: Vec<i32>, target: i32) -> bool {
    if nums[0] == target { return true }
    let mut rot = 0;
    for i in 0..nums.len() - 1 {
        if nums[i] > nums[i+1] { break; }
        if nums[i+1] == target { return true }
        rot = i+1;
    }
    // NOTE: find target using binary search with possible rotation
    let (mut l, mut r) = (rot + 1, nums.len());
    while r as isize - l as isize > 0 {
        let mid = (l + r) / 2;
        if nums[mid] == target { return true }
        if nums[mid] < target {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(search(vec![4,5,6,7,0,1,2], 0), true);
        assert_eq!(search(vec![4,5,6,7,0,1,2], 2), true);
        assert_eq!(search(vec![2,2,2,2,2,2,2], 3), false);
        assert_eq!(search(vec![1,1,1,1,1,1,1,1,1,13,1,1,1,1,1,1,1,1,1,1,1,1], 13), true);
        assert_eq!(search(vec![2,2,2,2,2,3,2], 3), true);
        assert_eq!(search(vec![1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1], 2), true);
        assert_eq!(search(vec![2,2,2,2,2,2,2], 2), true);
        assert_eq!(search(vec![3,2,2,2,2,2,2], 3), true);
        assert_eq!(search(vec![2,2,2,3,2,2,2], 3), true);
        assert_eq!(search(vec![2,5,6,0,0,1,2], 3), false);
        assert_eq!(search(vec![4,5,6,7,0,1,2], 0), true);
        assert_eq!(search(vec![5,1,3], 5), true);
        assert_eq!(search(vec![1], 1), true);
        assert_eq!(search(vec![0,1,2,3], 0), true);
        assert_eq!(search(vec![0,1,2,3], 4), false);
        assert_eq!(search(vec![4,5,6,7,0,1,2], 4), true);
        assert_eq!(search(vec![4,5,6,7,0,1,2], 3), false);
        assert_eq!(search(vec![1], 0), false);
    }
}
use crate::leetcode::print_2d_vec;

pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let len1 = nums1.len() + 1;
    let len2 = nums2.len() + 1;
    let mut dp: Vec<Vec<usize>> = vec![vec![0; len2]; len1];
    let mut ans = 0;
    for i in 1..len1 {
        for j in 1..len2 {
            if nums1[i - 1] == nums2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                if ans < dp[i][j] {
                    ans = dp[i][j];
                }
            }
        }
    }
    print_2d_vec(dp);
    ans as i32
}

#[test]
fn test() {
    assert_eq!(find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]), 3);
    assert_eq!(find_length(vec![9, 1, 2, 6, 6], vec![3, 4, 6, 6, 6]), 2);
    assert_eq!(find_length(vec![1, 2], vec![1, 2]), 2);
    assert_eq!(find_length(vec![6, 6, 6, 1, 6, 6, 6], vec![1, 1, 6, 6, 6]), 4);
    assert_eq!(find_length(vec![9, 1, 2, 6, 6], vec![3, 4, 6, 6, 6]), 2);
    assert_eq!(find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]), 5);
    assert_eq!(find_length(vec![6, 3, 2, 6, 6], vec![6, 3, 2, 1, 1]), 3);
    assert_eq!(find_length(vec![1, 1, 2, 2, 2], vec![3, 3, 3, 1, 1]), 2);
    assert_eq!(find_length(vec![1], vec![3]), 0);
    assert_eq!(find_length(vec![1], vec![1]), 1);
    assert_eq!(find_length(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]), 0);
    assert_eq!(find_length(vec![1, 2, 3, 4, 5], vec![6, 7, 3, 9, 10]), 1);
    assert_eq!(find_length(vec![1], vec![6, 7, 3, 9, 10]), 0);
    assert_eq!(find_length(vec![1, 2, 3, 4, 5], vec![2, 3]), 2);
    assert_eq!(find_length(vec![3, 3, 3, 3], vec![2, 3, 3, 3]), 3);
}

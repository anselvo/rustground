// TODO solve this one
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (mut l1, mut r1) = (0, nums1.len());
    let (mut l2, mut r2) = (0, nums2.len());
    while r1 - l1 > 0 || r2 - l2 > 0 {
        let mid1 = if r1 - l1 == 0 { l1 - 1 } else  { (l1 + r1) / 2 };
        let mid2 = if r2 - l2 == 0 { l2 - 1 } else { (l2 + r2) / 2 };
        print!("{} {} = {} || {} {} = {}", l1, r1, mid1, l2, r2, mid2);
        println!(" --- {} {}", nums1[mid1], nums2[mid2]);
        if nums1[mid1] < nums2[mid2] {
            l1 = mid1+1; r2 = mid2;
        } else {
            r1 = mid1; l2 = mid2+1;
        }
    }
    println!("{} {}", (l1 + r1) / 2, (l2 + r2) / 2);
    if (nums1.len() + nums2.len()) % 2 == 0 {
        ((nums1[r1-1] + nums2[r2-1]) / 2) as f64
    } else {
        if nums1[r1-1] > nums2[r2-1] { nums1[r1-1] as f64 }
        else { nums2[r2-1] as f64 }
    }
}

// 1 2 3 4 | 5 6 7 8

// 1 2 | 3 4 5 6 7 8

// 1   3   5   7
//   2   4   6   8

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_median_sorted_arrays(vec![1, 2, 3, 4], vec![5, 6, 7, 8]), 3.5);
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![1, 2, 3, 4], vec![5, 6, 7, 8]), 3.5);
        assert_eq!(find_median_sorted_arrays(vec![5, 6, 7, 8], vec![1, 2, 3, 4]), 3.5);
    }
}
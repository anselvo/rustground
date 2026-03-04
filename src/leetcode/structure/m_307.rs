pub struct NumArray {
    sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `& mut self` instead.
 */
impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        assert!(nums.len() > 0, "array should have at least one element");
        let mut sum = vec![nums[0]];
        for i in 1..nums.len() {
            sum.push(nums[i] + sum[i-1]);
        }
        Self { sum }
    }

    pub fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let tmp = if index == 0 { self.sum[index] - val } else { self.sum[index] - self.sum[index-1] - val };
        for i in index..self.sum.len() {
            self.sum[i] -= tmp
        }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 { return self.sum[right as usize] }
        self.sum[right as usize] - self.sum[(left-1) as usize]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */
#[cfg(test)]
mod tests {
    use crate::leetcode::structure::m_307::NumArray;

    #[test]
    fn should_create_array() {
        let ar = NumArray::new(vec![1, 3, 5]);
        assert_eq!(ar.sum_range(0, 2), 9);
    }

    #[test]
    fn should_update_element() {
        let mut ar = NumArray::new(vec![1, 3, 5]);
        ar.update(1, 2);
        assert_eq!(ar.sum_range(0, 2), 8);
        assert_eq!(ar.sum_range(0, 1), 3);
    }

    #[test]
    fn should_count_sum_ranges() {
        let ar = NumArray::new(vec![1, 3, 5, 6, 4, 3, 2, 1]);
        assert_eq!(ar.sum_range(0, 2), 9);
        assert_eq!(ar.sum_range(0, 3), 15);
        assert_eq!(ar.sum_range(0, 7), 25);
        assert_eq!(ar.sum_range(6, 7), 3);
        assert_eq!(ar.sum_range(3, 4), 10);
        assert_eq!(ar.sum_range(0, 0), 1);
        assert_eq!(ar.sum_range(7, 7), 1);
        assert_eq!(ar.sum_range(3, 3), 6);
    }
}
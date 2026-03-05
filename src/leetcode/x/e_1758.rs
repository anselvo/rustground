use std::cmp::min;

pub fn min_operations(s: String) -> i32 {
    let c = s.as_bytes();
    let mut ans = 0;
    for i in 0..c.len() {
        if c[i] - ('0' as u8) == (i % 2) as u8 {
            ans += 1;
        }
    }
    min(ans, c.len() as i32 - ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(min_operations("0100".to_string()), 1);
        assert_eq!(min_operations("1111".to_string()), 2);
        assert_eq!(min_operations("0110".to_string()), 2);
        assert_eq!(min_operations("1001".to_string()), 2);
        assert_eq!(min_operations("1011".to_string()), 1);
        assert_eq!(min_operations("0011".to_string()), 2);
        assert_eq!(min_operations("1110".to_string()), 1);
        assert_eq!(min_operations("0111".to_string()), 1);
        assert_eq!(min_operations("01010".to_string()), 0);
        assert_eq!(min_operations("01011".to_string()), 1);
        assert_eq!(min_operations("11110".to_string()), 2);
        assert_eq!(min_operations("111111".to_string()), 3);
        assert_eq!(min_operations("10".to_string()), 0);
        assert_eq!(min_operations("1".to_string()), 0);
    }
}

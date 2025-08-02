pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 { return 0; }
    if x == 1 { return 1; }
    let (mut l, mut r) = (0, x);


    while l < r {
        let half = (l + r) / 2;
        println!("{} {} {}", half, l, r);
        if x as i64 <= half as i64 * half as i64 {
            r = half;
        } else {
            l = half + 1;
        }
    }

    l
}

pub fn my_sqrt_bruteforce(x: i32) -> i32 {
    if x == 0 { return 0; }
    for i in 1..x {
        let res = (i as i64) * (i as i64);
        if res > x as i64 {
            return i - 1;
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(my_sqrt(0), 0);
        // assert_eq!(my_sqrt(1), 1);
        // assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(8), 2);
        assert_eq!(my_sqrt(9), 3);
        assert_eq!(my_sqrt(25), 5);
        assert_eq!(my_sqrt(26), 5);
        assert_eq!(my_sqrt(2147395600), 46340);
    }

    #[test]
    fn test_bruteforce() {
        assert_eq!(my_sqrt_bruteforce(0), 0);
        assert_eq!(my_sqrt_bruteforce(1), 1);
        assert_eq!(my_sqrt_bruteforce(4), 2);
        assert_eq!(my_sqrt_bruteforce(8), 2);
        assert_eq!(my_sqrt_bruteforce(9), 3);
        assert_eq!(my_sqrt_bruteforce(25), 5);
        assert_eq!(my_sqrt_bruteforce(26), 5);
        assert_eq!(my_sqrt_bruteforce(2147395600), 46340);
    }
}

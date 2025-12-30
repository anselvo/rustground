// NOTE: a recursion version of fib doesn't pass time limits
pub fn climb_stairs(mut n: i32) -> i32 {
    let mut sum = 1;
    let mut prev_sum = 1;
    while n != 1 {
        let tmp = sum;
        sum += prev_sum;
        prev_sum = tmp;
        n -= 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
        assert_eq!(climb_stairs(5), 8);
    }
}

/*

1 1 1 1
1 1 2
1 2 1
2 1 1
2 2

1 1 1 1 1
2 1 1 1
1 2 1 1
1 1 2 1
1 1 1 2
2 2 1
1 2 2
2 1 2

*/
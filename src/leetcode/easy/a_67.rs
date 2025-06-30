pub fn add_binary(a: String, b: String) -> String {
    let size = a.len().max(b.len());
    let na = format!("{:0>size$}", a);
    let nb = format!("{:0>size$}", b);
    let mut res = String::new();
    
    let mut carry = 0;
    for (ca, cb) in na.chars().rev().zip(nb.chars().rev()) {
        let da = ca.to_digit(2).unwrap();
        let db = cb.to_digit(2).unwrap();

        let dres = da + db + carry;
        match dres {
            0 => { res.push('0'); carry = 0; }
            1 => { res.push('1'); carry = 0; }
            2 => { res.push('0'); carry = 1; }
            3 => { res.push('1'); carry = 1; }
            _ => {}
        }
    }
    
    if carry == 1 {
        res.push('1');
    }

    res.chars().rev().collect()
}

pub fn add_binary_u128(a: String, b: String) -> String {
    let res_a = u128::from_str_radix(&*a, 2).unwrap();
    let res_b = u128::from_str_radix(&*b, 2).unwrap();
    let res = res_a + res_b;
    format!("{:b}", res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(add_binary("11".to_string(), "1".to_string()), "100");
        assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101");
        assert_eq!(add_binary("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string(), "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string()), "110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000");
        assert_eq!(add_binary("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(), "1".to_string()), "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");

    }

    #[test]
    fn test_u128() {
        assert_eq!(add_binary_u128("11".to_string(), "1".to_string()), "100");
        assert_eq!(add_binary_u128("1010".to_string(), "1011".to_string()), "10101");
        assert_eq!(add_binary_u128("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string(), "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string()), "110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000");
    }
}

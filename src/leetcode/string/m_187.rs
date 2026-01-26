use std::collections::HashMap;

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    if s.len() < 10 { return vec![] }
    let mut counts = HashMap::<&str, i32>::new();
    for i in 10..=s.len() {
        counts.entry(&s[i-10..i]).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut ans = vec![];
    for (str, count) in counts {
        if count > 1 {
            ans.push((&str).to_string());
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string()).sort_unstable(), vec!["AAAAACCCCC","CCCCCAAAAA"].sort_unstable());
        assert_eq!(find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string()), vec!["AAAAAAAAAA"]);
        assert_eq!(find_repeated_dna_sequences("A".to_string()), Vec::<String>::new());
        assert_eq!(find_repeated_dna_sequences("AAAAAAAAAAA".to_string()), vec!["AAAAAAAAAA"]);
        assert_eq!(find_repeated_dna_sequences("AAAAAAAAAA".to_string()), Vec::<String>::new());
        assert_eq!(find_repeated_dna_sequences("ACGTACGTAC".to_string()), Vec::<String>::new());
    }
}
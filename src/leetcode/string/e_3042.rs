pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
    if words.len() < 2 { return 0; }
    let mut ans = 0;
    for i in 1..words.len() {
        for j in i..words.len() {
            if is_prefix_and_suffix(&words[i - 1], &words[j]) {
                ans += 1;
            }
        }
    }
    ans
}

fn is_prefix_and_suffix(w1: &String, w2: &String) -> bool {
    if w1.len() > w2.len() { return false }
    let h1: i32 = w1.bytes().map(|b| b as i32).fold(0, |acc, b| acc * 31 + b);
    let suf: i32 = w2[..w1.len()].bytes().map(|b| b as i32).fold(0, |acc,  b| acc * 31 + b);
    let pre: i32 = w2[w2.len()-w1.len()..].bytes().map(|b| b as i32).fold(0, |acc,  b| acc * 31 + b);
    h1 == suf && h1 == pre
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(count_prefix_suffix_pairs(vec!["abc".to_string(),"bac".to_string(),"aa".to_string()]), 0);
        assert_eq!(count_prefix_suffix_pairs(vec!["a".to_string(),"aba".to_string(),"ababa".to_string(),"aa".to_string()]), 4);
        assert_eq!(count_prefix_suffix_pairs(vec!["pa".to_string(),"papa".to_string(),"ma".to_string(),"mama".to_string()]), 2);
        assert_eq!(count_prefix_suffix_pairs(vec!["abab".to_string(),"ab".to_string()]), 0);
        assert_eq!(count_prefix_suffix_pairs(vec!["sad".to_string()]), 0);
    }
}

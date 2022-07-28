pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut start, mut max_length) = (0, 0);
        let mut seen = HashMap::new();
        for (idx, ch) in s.chars().enumerate() {
            if let Some(last_seen) = seen.insert(ch, idx) {
                start = start.max(last_seen + 1);
            }
            max_length = max_length.max(idx - start + 1);
        }

        max_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("tmmzuxt".to_string()),
            5
        );
    }
}
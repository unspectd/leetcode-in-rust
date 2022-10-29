pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let (first, rest) = strs.split_first().unwrap();
        for (idx, ch) in first.chars().enumerate() {
            if !rest
                .into_iter()
                .all(|s| s.chars().nth(idx).unwrap_or_default() == ch)
            {
                return first[..idx].to_owned();
            }
        }

        first.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::longest_common_prefix(vec_string!["flower", "flow", "flight"]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec_string!["dog", "racecar", "car"]),
            ""
        );
    }
}
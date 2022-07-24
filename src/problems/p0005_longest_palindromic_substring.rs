pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let mut answer = String::new();
        for i in 0..bytes.len() {
            if i > 0 && bytes[i - 1] == bytes[i] {
                continue;
            }

            let (mut lo, mut hi) = (i, i);
            while hi < bytes.len() - 1 && bytes[hi] == bytes[hi + 1] {
                hi += 1;
            }

            while lo > 0 && hi < bytes.len() - 1 && bytes[lo - 1] == bytes[hi + 1] {
                lo -= 1;
                hi += 1;
            }

            if hi - lo + 1 > answer.len() {
                answer = (&s[lo..=hi]).to_string()
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }
}
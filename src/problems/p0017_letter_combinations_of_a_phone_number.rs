pub struct Solution;

use std::ops::RangeInclusive;

impl Solution {
    const MAPPING: [RangeInclusive<u8>; 8] = [
        b'a'..=b'c',
        b'd'..=b'f',
        b'g'..=b'i',
        b'j'..=b'l',
        b'm'..=b'o',
        b'p'..=b's',
        b't'..=b'v',
        b'w'..=b'z',
    ];

    pub fn letter_combinations(digits: String) -> Vec<String> {
        fn backtrack(buffer: &mut String, digits: &str, results: &mut Vec<String>) {
            if digits.is_empty() {
                results.push(buffer.clone());
                return;
            }

            let idx: usize = (digits.chars().nth(0).unwrap() as u8 - b'2') as usize;
            for ch in Solution::MAPPING[idx].clone() {
                buffer.push(ch as char);
                backtrack(buffer, &digits[1..], results);
                buffer.pop();
            }
        }

        let mut results = Vec::new();

        if !digits.is_empty() {
            let mut buffer = String::with_capacity(digits.len());
            backtrack(&mut buffer, digits.as_str(), &mut results);
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (digits, result)
        struct TestCase(&'static str, Vec<&'static str>);
        let test_cases = [
            TestCase("", vec![]),
            TestCase("2", vec!["a", "b", "c"]),
            TestCase(
                "23",
                vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            ),
        ];

        for test_case in test_cases {
            let actual = Solution::letter_combinations(test_case.0.to_string());
            let expected = test_case
                .1
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            assert_eq!(actual, expected);
        }
    }
}
pub struct Solution;

use std::cmp::min;

// todo: add memoization someday
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn is_valid(s: &str) -> bool {
            match s.len() {
                1 => true,
                2 => s[0..1] != *"0",
                3 => s[0..1] != *"0" && s[0..=2] <= *"255",
                _ => unreachable!(),
            }
        }

        fn backtrace<'a>(
            parts: &mut Vec<&'a str>,
            str: &'a str,
            start: usize,
            results: &mut Vec<String>,
        ) {
            if parts.capacity() == parts.len() {
                if start == str.len() {
                    let ip = parts
                        .iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<_>>()
                        .join(".");
                    results.push(ip);
                }
                return;
            }

            let lengths = (1..=min(3, str.len() - start))
                .into_iter()
                .filter(|len| is_valid(&str[start..start + len]));
            for len in lengths {
                parts.push(&str[start..start + len]);
                backtrace(parts, str, start + len, results);
                parts.pop();
            }
        }

        let mut results = Vec::new();
        let mut buffer = Vec::with_capacity(4);
        backtrace(&mut buffer, &s, 0, &mut results);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (s, result)
        struct TestCase(&'static str, Vec<&'static str>);
        let test_cases = [
            TestCase("000256", vec![]),
            TestCase("25525511135", vec!["255.255.11.135", "255.255.111.35"]),
            TestCase("0000", vec!["0.0.0.0"]),
            TestCase(
                "101023",
                vec![
                    "1.0.10.23",
                    "1.0.102.3",
                    "10.1.0.23",
                    "10.10.2.3",
                    "101.0.2.3",
                ],
            ),
        ];

        for test_case in test_cases {
            let actual = Solution::restore_ip_addresses(test_case.0.to_string());
            let expected = test_case
                .1
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            assert_eq!(actual, expected);
        }
    }
}
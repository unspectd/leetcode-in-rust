pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(buffer: &mut String, to_open: i32, to_close: i32, results: &mut Vec<String>) {
            if to_open == 0 && to_close == 0 {
                results.push(buffer.clone());
                return;
            }

            if to_open > 0 {
                buffer.push(b'(' as char);
                backtrack(buffer, to_open - 1, to_close, results);
                buffer.pop();
            }

            if to_close > to_open {
                buffer.push(b')' as char);
                backtrack(buffer, to_open, to_close - 1, results);
                buffer.pop();
            }
        }

        let mut results = Vec::new();
        let mut buffer = String::with_capacity(n as usize * 2);
        backtrack(&mut buffer, n, n, &mut results);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (n, result)
        struct TestCase(i32, Vec<&'static str>);
        let test_cases = [
            TestCase(1, vec!["()"]),
            TestCase(2, vec!["(())", "()()"]),
            TestCase(3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
        ];

        for test_case in test_cases {
            let actual = Solution::generate_parenthesis(test_case.0);
            let expected = test_case
                .1
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            assert_eq!(actual, expected);
        }
    }
}
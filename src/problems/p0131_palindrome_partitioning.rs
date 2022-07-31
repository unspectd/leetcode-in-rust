pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn backtrack(
            buffer: &mut Vec<String>,
            cache: &mut Vec<Vec<bool>>,
            s: &[u8],
            start: usize,
            results: &mut Vec<Vec<String>>,
        ) {
            if start >= s.len() {
                results.push(buffer.clone());
                return;
            }

            for end in start..s.len() {
                if s[start] == s[end] && (end - start <= 2 || cache[start + 1][end - 1]) {
                    cache[start][end] = true;
                    buffer.push(String::from_utf8((&s[start..end + 1]).to_owned()).unwrap());
                    backtrack(buffer, cache, s, end + 1, results);
                    buffer.pop();
                }
            }
        }

        let mut buffer = Vec::with_capacity(s.len());
        let mut results = Vec::new();
        let mut cache = vec![vec![false; s.len()]; s.len()];
        backtrack(&mut buffer, &mut cache, s.as_bytes(), 0, &mut results);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (s, result)
        struct TestCase(&'static str, Vec<Vec<&'static str>>);
        let test_cases = [
            TestCase("aab", vec![vec!["a", "a", "b"], vec!["aa", "b"]]),
            TestCase("a", vec![vec!["a"]]),
        ];

        for test_case in test_cases {
            // todo: refactor later
            let mut actual = Solution::partition(test_case.0.to_string());
            let mut expected = test_case
                .1
                .iter()
                .map(|x| x.iter().map(|y| y.to_string()).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            // todo: refactor later
            fn sort(v: &mut Vec<Vec<String>>) {
                v.iter_mut().for_each(|strings| strings.sort());
                v.sort_by(|a, b| {
                    let joined_a = a.join("|");
                    let joined_b = b.join("|");
                    joined_a.partial_cmp(&joined_b).unwrap()
                })
            }

            sort(&mut actual);
            sort(&mut expected);
            assert_eq!(actual, expected);
        }
    }
}
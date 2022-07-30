pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .fold(HashMap::new(), |mut groups, s| {
                let key = s.as_bytes().iter().fold([0usize; 26], |mut k, b| {
                    k[(*b - b'a') as usize] += 1;
                    k
                });
                groups.entry(key).or_insert_with(|| vec![]).push(s);
                groups
            })
            .into_values()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (strs, result)
        struct TestCase(Vec<&'static str>, Vec<Vec<&'static str>>);
        let test_cases = [
            TestCase(
                vec!["eat", "tea", "tan", "ate", "nat", "bat"],
                vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]],
            ),
            TestCase(vec![""], vec![vec![""]]),
            TestCase(vec!["a"], vec![vec!["a"]]),
        ];

        for test_case in test_cases {
            // todo: refactor later
            let mut actual =
                Solution::group_anagrams(test_case.0.iter().map(|x| x.to_string()).collect());
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
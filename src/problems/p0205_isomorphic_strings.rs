pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut seen = HashSet::new();
        let mut mappings = HashMap::new();
        s.chars()
            .zip(t.chars())
            .all(|(s, t)| match mappings.get(&s) {
                Some(mapped) => *mapped == t,
                None => {
                    mappings.insert(s, t);
                    seen.insert(t)
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_isomorphic("egg".to_string(), "add".to_string()),
            true
        );
        assert_eq!(
            Solution::is_isomorphic("egr".to_string(), "add".to_string()),
            false
        );
        assert_eq!(
            Solution::is_isomorphic("egg".to_string(), "ads".to_string()),
            false
        );
    }
}
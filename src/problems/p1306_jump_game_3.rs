pub struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut visited = vec![false; arr.len()];
        let mut queue = vec![start as usize];

        while let Some(idx) = queue.pop() {
            if visited[idx] {
                continue;
            }

            let dist = arr[idx] as usize;
            if dist == 0 {
                return true;
            }

            visited[idx] = true;
            if idx >= dist {
                queue.push(idx - dist);
            }
            if idx + dist < arr.len() {
                queue.push(idx + dist);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (arr, start, result)
        struct TestCase(Vec<i32>, i32, bool);
        let test_cases = [
            TestCase(vec![4, 2, 3, 0, 3, 1, 2], 5, true),
            TestCase(vec![4, 2, 3, 0, 3, 1, 2], 0, true),
            TestCase(vec![3, 0, 2, 1, 2], 2, false),
        ];

        for test_case in test_cases {
            assert_eq!(Solution::can_reach(test_case.0, test_case.1), test_case.2);
        }
    }
}
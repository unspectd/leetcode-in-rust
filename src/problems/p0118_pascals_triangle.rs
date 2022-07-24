pub struct Solution;

impl Solution {
    pub fn generate(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut triangle = Vec::<Vec<_>>::with_capacity(n);
        for i in 0..n {
            let row = match i {
                0 => vec![1],
                1 => vec![1, 1],
                _ => {
                    let mut row = vec![0; i + 1];
                    (row[0], row[i]) = (1, 1);

                    let prev_row = &triangle[i - 1];
                    for j in 1..i {
                        row[j] = prev_row[j - 1] + prev_row[j];
                    }
                    row
                }
            };
            triangle.push(row);
        }

        triangle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (n, result)
        struct TestCase(i32, Vec<Vec<i32>>);
        let test_cases = [
            TestCase(1, vec![vec![1]]),
            TestCase(
                5,
                vec![
                    vec![1],
                    vec![1, 1],
                    vec![1, 2, 1],
                    vec![1, 3, 3, 1],
                    vec![1, 4, 6, 4, 1],
                ],
            ),
        ];

        for test_case in test_cases {
            assert_eq!(Solution::generate(test_case.0), test_case.1);
        }
    }
}
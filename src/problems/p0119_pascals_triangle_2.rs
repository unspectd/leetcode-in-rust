pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let len = row_index as usize + 1;
        let mut row = vec![0; len];
        row[0] = 1;

        for n in 1..len {
            for i in (1..=n).rev() {
                row[i] += row[i - 1]
            }
        }

        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }
}
struct Solution;

impl Solution {
    pub fn running_sum_v1(nums: Vec<i32>) -> Vec<i32> {
        let mut values: Vec<i32> = Vec::new();

        for (i, x) in nums.iter().enumerate() {
            if i == 0 {
                values.push(*x);
            } else {
                values.push(*x + values[i - 1]);
            }
        }

        values
    }

    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut val = 0;
        nums.iter().map(|x| {val += x; val}).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn base() {
        assert_eq!(Solution::running_sum((1..5).collect()), vec![1, 3, 6, 10]);
    }
}

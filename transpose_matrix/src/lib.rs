struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let r = matrix.len();
        let c = matrix[0].len();
        let mut ans = vec![vec![0; r]; c];

        for (y, i) in matrix.iter().enumerate() {
            for (x, j) in matrix[0].iter().enumerate() {
                ans[x][y] = matrix[y][x];
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
    }
}

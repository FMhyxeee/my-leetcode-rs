use crate::Solution;


// 最小下降路径和
// 使用动态规划进行求解

impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());

        for i in 1..m {
            (0..n).for_each(|j| 
                matrix[i][j] += *matrix[i - 1][(j.max(1)-1)..(j.min(n-2)+2)].iter().min().unwrap());
        }

        *matrix[m - 1].iter().min().unwrap()

    }
}
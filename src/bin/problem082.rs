// Problem #82: Path Sum: Three Ways
// https://projecteuler.net/problem=82

fn min_path_sum_three_ways(grid: Vec<Vec<u32>>) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut dp = vec![vec![0; cols]; rows];
    for i in 0..rows {
        dp[i][0] = grid[i][0];
    }

    for j in 1..cols {
        for i in 0..rows {
            dp[i][j] = dp[i][j - 1] + grid[i][j];
        }

        for i in 1..rows {
            dp[i][j] = dp[i][j].min(dp[i - 1][j] + grid[i][j]);
        }

        for i in (0..rows - 1).rev() {
            dp[i][j] = dp[i][j].min(dp[i + 1][j] + grid[i][j]);
        }
    }

    let mut result = dp[0][cols - 1];
    for i in 1..rows {
        result = result.min(dp[i][cols - 1]);
    }

    result
}

fn main() {
    let grid = vec![
        vec![131, 673, 234, 103, 18],
        vec![201, 96, 342, 965, 150],
        vec![630, 803, 746, 422, 111],
        vec![537, 699, 497, 121, 956],
        vec![805, 732, 524, 37, 331],
    ];

    let result = min_path_sum_three_ways(grid);
    println!("The minimum path sum is {}", result);
}

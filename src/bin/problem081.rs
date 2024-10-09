// Problem #81: Path Sum: Two Ways
// https://projecteuler.net/problem=81

fn min_path_sum(grid: Vec<Vec<u32>>) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dp = vec![vec![0; cols]; rows];

    dp[0][0] = grid[0][0];

    for j in 1..cols {
        dp[0][j] = dp[0][j - 1] + grid[0][j];
    }

    for i in 1..rows {
        dp[i][0] = dp[i - 1][0] + grid[i][0];
    }

    for i in 1..rows {
        for j in 1..cols {
            dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + grid[i][j];
        }
    }

    dp[rows - 1][cols - 1]
}

fn main() {
    let grid = vec![
        vec![131, 673, 234, 103, 18],
        vec![201, 96, 342, 965, 150],
        vec![630, 803, 746, 422, 111],
        vec![537, 699, 497, 121, 956],
        vec![805, 732, 524, 37, 331],
    ];

    let result = min_path_sum(grid);
    println!("The minimum path sum is {}", result);
}

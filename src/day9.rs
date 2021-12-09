pub struct Day9;

impl crate::lib::DayInner<Day9, i32> for Day9 {
    fn day(&self) -> i32 {
        9
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let mut grid: Vec<Vec<i32>> = vec![];

        let lines: Vec<&str> = input.lines().collect();
        let grid_width = lines[0].len();
        let full_wdith = grid_width + 2;
        grid.push(vec![10; full_wdith]);

        for line in lines {
            let mut new_row = vec![10; full_wdith];
            let digits: Vec<i32> = line
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect();
            new_row[1..grid_width + 1].clone_from_slice(&digits[0..grid_width]);
            grid.push(new_row);
        }
        grid.push(vec![10; full_wdith]);

        let mut val = 0;
        for jj in 1..grid_width + 1 {
            for ii in 1..grid.len() - 1 {
                if grid[ii][jj] < grid[ii - 1][jj]
                    && grid[ii][jj] < grid[ii + 1][jj]
                    && grid[ii][jj] < grid[ii][jj - 1]
                    && grid[ii][jj] < grid[ii][jj + 1]
                {
                    val += grid[ii][jj] + 1;
                }
            }
        }

        (val, 0)
    }
}

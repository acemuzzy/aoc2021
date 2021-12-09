pub struct Day9;

struct Grid {
    grid: Vec<Vec<i32>>,
    basins: Vec<Vec<Option<i32>>>,
}

impl crate::lib::DayInner<Day9, i32> for Day9 {
    fn day(&self) -> i32 {
        9
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let mut grid = Grid {
            grid: vec![],
            basins: vec![],
        };
        let mut risk = 0;
        let mut basin_sizes: Vec<i32> = vec![];

        let lines: Vec<&str> = input.lines().collect();
        let grid_width = lines[0].len();
        let full_wdith = grid_width + 2;
        grid.grid.push(vec![10; full_wdith]);
        grid.basins.push(vec![None; full_wdith]);

        for line in lines {
            let mut new_row = vec![10; full_wdith];
            let digits: Vec<i32> = line
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect();
            new_row[1..grid_width + 1].clone_from_slice(&digits[0..grid_width]);
            grid.grid.push(new_row);
            grid.basins.push(vec![None; full_wdith]);
        }
        grid.grid.push(vec![10; full_wdith]);
        grid.basins.push(vec![None; full_wdith]);

        // Work out risk value (for Part 1)
        for jj in 1..grid_width + 1 {
            for ii in 1..grid.grid.len() - 1 {
                if grid.grid[ii][jj] < grid.grid[ii - 1][jj]
                    && grid.grid[ii][jj] < grid.grid[ii + 1][jj]
                    && grid.grid[ii][jj] < grid.grid[ii][jj - 1]
                    && grid.grid[ii][jj] < grid.grid[ii][jj + 1]
                {
                    risk += grid.grid[ii][jj] + 1;
                }
            }
        }

        // Sort out basins (for Part 2)
        let mut basin_index = 0;
        for jj in 1..grid_width + 1 {
            for ii in 1..grid.grid.len() - 1 {
                if grid.grid[ii][jj] == 9 {
                    continue;
                }

                if grid.basins[ii][jj].is_some() {
                    continue;
                }

                // OK, new location, not yet in a basin.  Let's fill!
                let mut open_list = vec![(ii, jj)];
                let mut count = 0;

                while !open_list.is_empty() {
                    let (x, y) = open_list.pop().unwrap();
                    // We could have added the entry multiple times
                    // Should check for contains, but whatevs it's finite
                    if grid.basins[x][y].is_some() {
                        continue;
                    }
                    count += 1;
                    grid.basins[x][y] = Some(basin_index);

                    if grid.grid[x - 1][y] < 9 && grid.basins[x - 1][y].is_none() {
                        open_list.push((x - 1, y));
                    }
                    if grid.grid[x + 1][y] < 9 && grid.basins[x + 1][y].is_none() {
                        open_list.push((x + 1, y));
                    }
                    if grid.grid[x][y - 1] < 9 && grid.basins[x][y - 1].is_none() {
                        open_list.push((x, y - 1));
                    }
                    if grid.grid[x][y + 1] < 9 && grid.basins[x][y + 1].is_none() {
                        open_list.push((x, y + 1));
                    }
                }

                basin_sizes.push(count);
                basin_index += 1;
            }
        }

        // for row in grid.basins {
        //     let output: String = row.iter().map(|x| match x {
        //         Some(d) => {
        //             match (d % 4) {
        //               0 => ".",
        //               1 => "\\",
        //               2 => "%",
        //               _ => "/",
        //             }.to_string()
        //         },
        //         _ => "X".to_string(),
        //     }).collect::<String>();
        //     println!("{}", output);
        // }

        basin_sizes.sort_by(|a, b| b.cmp(a));
        let basin = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];

        (risk, basin)
    }
}

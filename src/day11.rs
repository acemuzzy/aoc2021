pub struct Day11;

struct Grid {
    grid: Vec<Vec<i32>>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.grid.iter() {
            let output: String = row.iter().map(|d| d.to_string()).collect();
            writeln!(f, "{}", output)?;
        }
        Ok(())
    }
}

impl Grid {
    fn increment(&mut self, x: i32, y: i32) {
        if (0..10).contains(&x) && (0..10).contains(&y) && self.grid[y as usize][x as usize] > 0 {
            self.grid[y as usize][x as usize] += 1;
        }
    }
    fn next_gen(&self) -> (Self, i32) {
        let mut new_grid: Vec<Vec<i32>> = vec![];
        let mut flashes = 0;

        for row in self.grid.iter() {
            new_grid.push(row.iter().map(|x| x + 1).collect())
        }
        let mut new_grid = Grid { grid: new_grid };
        loop {
            let mut pass_flashes = 0i32;

            for yy in 0i32..10i32 {
                for xx in 0i32..10i32 {
                    if new_grid.grid[yy as usize][xx as usize] > 9 {
                        pass_flashes += 1;
                        new_grid.grid[yy as usize][xx as usize] = 0;
                        for yyy in yy - 1..yy + 2 {
                            for xxx in xx - 1..xx + 2 {
                                new_grid.increment(xxx, yyy);
                            }
                        }
                    }
                }
            }

            // Quit if we've had no flashes.
            if pass_flashes == 0 {
                break;
            }

            flashes += pass_flashes;
        }

        (new_grid, flashes)
    }
}

impl crate::lib::DayInner<Day11, i32> for Day11 {
    fn day(&self) -> i32 {
        11
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let mut grid = Grid { grid: vec![] };

        let lines: Vec<&str> = input.lines().collect();
        for line in lines {
            let digits: Vec<i32> = line
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect();
            grid.grid.push(digits);
        }

        let mut flashes = 0;
        let mut count = 0;
        let mut first_flash = 0;

        loop {
            count += 1;
            let (gen_grid, gen_flashes) = grid.next_gen();
            if count <= 100 {
                flashes += gen_flashes;
            }
            if gen_flashes == 100 {
                first_flash = count;
            }

            if (count > 100) && gen_flashes == 100 {
                break;
            }
            grid = gen_grid;

            println!("\n{}", grid);
            println!("Gen: {}, Flashes: {}", count, flashes);
        }

        (flashes, first_flash)
    }
}

pub struct Day13;

struct Grid {
    grid: Vec<Vec<bool>>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.grid.iter() {
            let output: String = row.iter().map(|b| if *b { "#" } else { "." }).collect();
            writeln!(f, "{}", output)?;
        }
        Ok(())
    }
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> bool {
        if y < 0 || x < 0 || y >= self.grid.len() as i32 || x >= self.grid[0].len() as i32 {
            false
        } else {
            self.grid[y as usize][x as usize]
        }
    }

    fn foldalongxequals(&self, x: i32) -> Grid {
        let height = self.grid.len();
        let width = std::cmp::max(x, self.grid[0].len() as i32 - 1 - x);
        let mut grid: Vec<Vec<bool>> = vec![];

        for _ in 0..height {
            grid.push(vec![false; width as usize]);
        }

        for yy in 0..height {
            for xx in 0..width {
                grid[yy as usize][(width - 1 - xx) as usize] =
                    self.get(x - xx - 1, yy as i32) | self.get(x + xx + 1, yy as i32);
            }
        }

        Grid { grid }
    }

    fn foldalongyequals(&self, y: i32) -> Grid {
        let height: i32 = std::cmp::max(y, self.grid.len() as i32 - y - 1);
        let width: i32 = self.grid[0].len() as i32;
        let mut grid: Vec<Vec<bool>> = vec![];

        for _ in 0..height {
            grid.push(vec![false; width as usize]);
        }

        for yy in 0..height {
            for xx in 0..width {
                grid[(height - 1 - yy) as usize][xx as usize] =
                    self.get(xx, y - yy - 1) | self.get(xx, y + yy + 1);
            }
        }

        Grid { grid }
    }

    fn count(&self) -> i32 {
        self.grid
            .iter()
            .map(|x| x.iter().filter(|x| **x).count())
            .sum::<usize>() as i32
    }
}

impl crate::lib::DayInner<Day13, i32> for Day13 {
    fn day(&self) -> i32 {
        13
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let mut grid: Vec<Vec<bool>> = vec![];

        let mut width = 0;
        let mut height = 0;

        let mut folds: Vec<&str> = vec![];

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }

            if line.starts_with("fold") {
                folds.push(line);
                continue;
            }

            let coords = line
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            if coords[0] > width {
                width = coords[0];
            }
            if coords[1] > height {
                height = coords[1];
            }
        }

        width += 1;
        height += 1;

        for _ in 0..height {
            grid.push(vec![false; width as usize]);
        }

        for line in input.lines() {
            if line.is_empty() {
                break;
            }

            let coords = line
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();
            grid[coords[1]][coords[0]] = true;
        }

        let mut grid = Grid { grid };
        let mut part1: Option<i32> = None;

        for fold in folds {
            let d = fold.split('=').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();

            if fold.starts_with("fold along y") {
                grid = grid.foldalongyequals(d)
            } else {
                grid = grid.foldalongxequals(d);
            }

            if part1.is_none() {
                part1 = Some(grid.count());
            }
        }

        println!("{}", grid);

        (part1.unwrap(), 0)
    }
}

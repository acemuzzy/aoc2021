pub struct Day5;

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_diagonal(&self) -> bool {
        (self.start.1 - self.end.1).abs() == (self.start.0 - self.end.0).abs()
    }

    fn is_vertical_or_horizontal(&self) -> bool {
        self.is_vertical() | self.is_horizontal()
    }

    fn end_from_string(point: &str) -> (i32, i32) {
        let end_as_vec: Vec<i32> = point.split(',').map(|x| x.parse().unwrap()).collect();
        (end_as_vec[0], end_as_vec[1])
    }

    fn from_input_line(line: &str) -> Self {
        // Line of form e.g. "0,9 -> 5,9"
        let halves: Vec<&str> = line.split(" -> ").collect();
        Line {
            start: Self::end_from_string(halves[0]),
            end: Self::end_from_string(halves[1]),
        }
    }
}

impl crate::lib::DayInner<Day5> for Day5 {
    fn day(&self) -> i32 {
        5
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let lines: Vec<Line> = input.lines().map(Line::from_input_line).collect();
        let vertical_or_horizontal_lines: Vec<&Line> = lines
            .iter()
            .filter(|l| l.is_vertical_or_horizontal())
            .collect();
        let diagonal_lines: Vec<&Line> = lines
            .iter()
            .filter(|l| l.is_diagonal() & !l.is_vertical_or_horizontal())
            .collect();

        let xrange = lines
            .iter()
            .map(|l| std::cmp::max(l.start.0, l.end.0))
            .max()
            .unwrap();
        let yrange = lines
            .iter()
            .map(|l| std::cmp::max(l.start.1, l.end.1))
            .max()
            .unwrap();

        let mut grid: Vec<Vec<i32>> = vec![];
        for _ in 0..xrange + 1 {
            grid.push(vec![0; (yrange + 1) as usize]);
        }

        for line in &vertical_or_horizontal_lines {
            if line.is_horizontal() {
                let min = std::cmp::min(line.start.0, line.end.0);
                let max = std::cmp::max(line.start.0, line.end.0);
                for xx in min..max + 1 {
                    grid[xx as usize][line.start.1 as usize] += 1;
                }
            } else {
                let min = std::cmp::min(line.start.1, line.end.1);
                let max = std::cmp::max(line.start.1, line.end.1);
                for yy in min..max + 1 {
                    grid[line.start.0 as usize][yy as usize] += 1;
                }
            }
        }

        let overlaps1 = grid
            .iter()
            .map(|l| l.iter().filter(|p| **p > 1).count())
            .map(|x| x as i32)
            .sum::<i32>() as i32;
        println!("Overlaps1: {}", overlaps1);

        for line in &diagonal_lines {
            let len = (line.start.0 - line.end.0).abs();
            let vec = (
                (line.start.0 - line.end.0) / len,
                (line.start.1 - line.end.1) / len,
            );

            for dd in 0..len + 1 {
                grid[(line.end.0 + vec.0 * dd) as usize][(line.end.1 + vec.1 * dd) as usize] += 1;
            }
        }

        let overlaps2 = grid
            .iter()
            .map(|l| l.iter().filter(|p| **p > 1).count())
            .map(|x| x as i32)
            .sum::<i32>() as i32;
        println!("Overlaps2: {}", overlaps2);

        (overlaps1.into(), overlaps2.into())
    }
}

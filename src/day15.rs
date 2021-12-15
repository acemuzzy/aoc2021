pub struct Day15;

#[derive(Debug)]
struct Risks {
    individual: Vec<Vec<i32>>,
    cumulative: Vec<Vec<Option<i32>>>,
}

impl std::fmt::Display for Risks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.individual.iter() {
            let output: String = row.iter().map(|d| d.to_string()).collect();
            writeln!(f, "{}", output)?;
        }
        Ok(())
    }
}

impl Risks {
    fn get(&self, x: i32, y: i32) -> Option<i32> {
        if x < 0
            || y < 0
            || x >= self.individual[0].len() as i32
            || y >= self.individual.len() as i32
        {
            None
        } else {
            Some(self.individual[y as usize][x as usize])
        }
    }

    fn get_cumm(&self, x: i32, y: i32) -> Option<i32> {
        if x < 0
            || y < 0
            || x >= self.individual[0].len() as i32
            || y >= self.individual.len() as i32
        {
            unreachable!();
        } else {
            self.cumulative[y as usize][x as usize]
        }
    }

    fn walk(&mut self, x: i32, y: i32, cumulative: i32) {
        //println!("Visiting ({},{}), val {}", x, y, cumulative);
        self.cumulative[y as usize][x as usize] = Some(cumulative);

        // let delta = self.get(x - 1, y);
        // if let Some(d) = delta {
        //     if let Some(c) = self.get_cumm(x - 1, y) {
        //         if c > cumulative + d {
        //             self.walk(x - 1, y, cumulative + d);
        //         }
        //     } else {
        //         self.walk(x - 1, y, cumulative + d);
        //     }
        // }

        let delta = self.get(x + 1, y);
        if let Some(d) = delta {
            if let Some(c) = self.get_cumm(x + 1, y) {
                if c > cumulative + d {
                    self.walk(x + 1, y, cumulative + d);
                }
            } else {
                self.walk(x + 1, y, cumulative + d);
            }
        }

        // let delta = self.get(x, y - 1);
        // if let Some(d) = delta {
        //     if let Some(c) = self.get_cumm(x, y - 1) {
        //         if c > cumulative + d {
        //             self.walk(x, y - 1, cumulative + d);
        //         }
        //     } else {
        //         self.walk(x, y - 1, cumulative + d);
        //     }
        // }

        let delta = self.get(x, y + 1);
        if let Some(d) = delta {
            if let Some(c) = self.get_cumm(x, y + 1) {
                if c > cumulative + d {
                    self.walk(x, y + 1, cumulative + d);
                }
            } else {
                self.walk(x, y + 1, cumulative + d);
            }
        }
    }
}

impl crate::lib::DayInner<Day15, i32> for Day15 {
    fn day(&self) -> i32 {
        15
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let mut individual: Vec<Vec<i32>> = vec![];
        let mut cumulative: Vec<Vec<Option<i32>>> = vec![];

        for line in input.lines() {
            individual.push(
                line.chars()
                    .map(|d| d.to_string().parse::<i32>().unwrap())
                    .collect(),
            );
            cumulative.push(vec![None; line.len()]);
        }

        cumulative[0][0] = Some(0);

        let width = individual.len();
        let height = individual.len();

        for jj in 0..4 {
            for yy in 0..height {
                individual.push(
                    individual[height * jj + yy]
                        .iter()
                        .map(|x| if *x == 9 { 1 } else { x + 1 })
                        .collect(),
                );
                cumulative.push(vec![None; width]);
            }
        }

        for ii in 0..4 {
            for yy in 0..5 * height {
                let append: Vec<i32> = individual[yy][ii * width..(ii + 1) * width]
                    .iter()
                    .map(|x| if *x == 9 { 1 } else { x + 1 })
                    .collect::<Vec<i32>>();
                individual[yy].extend(&append);
                cumulative[yy].extend(vec![None; width]);
            }
        }

        assert_eq!(5 * width, individual[0].len());
        assert_eq!(5 * height, individual.len());

        let mut risks = Risks {
            individual,
            cumulative,
        };

        println!("{}", risks);

        risks.walk(0, 0, 0);

        let total1 = risks.cumulative[height - 1][width - 1].unwrap();
        let total2 = risks.cumulative[5 * height - 1][5 * width - 1].unwrap();

        (total1, total2)
    }
}

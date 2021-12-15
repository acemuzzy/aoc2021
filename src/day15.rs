use std::collections::HashSet;

pub struct Day15;

#[derive(Debug)]
struct Risks {
    individual: Vec<Vec<i32>>,
    cumulative: Vec<Vec<Option<i32>>>,
    best: Vec<Vec<Option<i32>>>,
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
    fn get(&self, x: i32, y: i32) -> i32 {
        // if x < 0
        //     || y < 0
        //     || x >= self.individual[0].len() as i32
        //     || y >= self.individual.len() as i32
        // {
        //     None
        // } else {
        //     Some(self.individual[y as usize][x as usize])
        // }
        self.individual[y as usize][x as usize]
    }

    fn get_cumm(&self, x: i32, y: i32) -> Option<i32> {
        self.cumulative[y as usize][x as usize]
    }

    fn walk(&mut self) {
        //println!("Visiting ({},{}), val {}", x, y, cumulative);

        // Start off with having visited nowhere
        let mut open_points = Vec::<(i32, i32)>::new();
        for yy in 0..self.cumulative.len() {
            for xx in 0..self.cumulative[0].len() {
                open_points.push((xx as i32, yy as i32));
            }
        }

        // While we've go points we've not visited
        // - Find the unvisited point with the lowest distance - we must have its optimal path
        // - Look at its unvisited neighbours, see if their current-best path can be created/bettered
        let mut count = 0;
        let width = self.individual[0].len();
        let height = self.individual.len();

        loop {
            count += 1;
            //let (x, y): (i32, i32) = *open_points.iter().nth(0).unwrap();
            let mut lowest = None;
            let (mut x, mut y): (i32, i32) = (0, 0);
            for (xx, yy) in open_points.iter() {
                if let Some(cum) = self.cumulative[*yy as usize][*xx as usize] {
                    if let Some(l) = lowest {
                        if cum < l {
                            x = *xx as i32;
                            y = *yy as i32;
                            lowest = Some(cum);
                        }
                    } else {
                        x = *xx as i32;
                        y = *yy as i32;
                        lowest = Some(cum);
                    }
                }
            }

            if lowest.is_none() {
                break;
            }

            open_points.retain(|(xx, yy)| (xx, yy) != (&x, &y));

            //println!("Checking point ({},{}) ({} of {})", x, y, count, self.cumulative.len() * self.cumulative[0].len());

            //open_points.remove(&(x as i32, y as i32));

            //self.cumulative[y as usize][x as usize] = Some(cumulative);
            let cumulative = lowest.unwrap(); //self.get_cumm(x, y).unwrap();
            self.best[y as usize][x as usize] = self.cumulative[y as usize][x as usize];

            // let delta = self.get(x - 1, y);
            // if let Some(d) = delta {
            //     if let Some(c) = self.get_cumm(x - 1, y) {
            //         if c > cumulative + d {
            //             self.cumulative[y as usize][(x - 1) as usize] = Some(cumulative + d);
            //             open_points.insert((x - 1, y));
            //         }
            //     } else {
            //         self.cumulative[y as usize][(x - 1) as usize] = Some(cumulative + d);
            //         open_points.insert((x - 1, y));
            //     }
            // }

            if x < (width as i32 - 1) {
                if self.best[y as usize][(x + 1) as usize].is_none() {
                    let d = self.individual[y as usize][(x + 1) as usize];
                    if let Some(c) = self.cumulative[y as usize][(x + 1) as usize] {
                        if c > cumulative + d {
                            self.cumulative[y as usize][(x + 1) as usize] = Some(cumulative + d);
                            //open_points.insert((x + 1, y));
                        }
                    } else {
                        self.cumulative[y as usize][(x + 1) as usize] = Some(cumulative + d);
                        // open_points.insert((x + 1, y));
                    }
                }
            }

            // let delta = self.get(x, y - 1);
            // if let Some(d) = delta {
            //     if let Some(c) = self.get_cumm(x, y - 1) {
            //         if c > cumulative + d {
            //             self.cumulative[(y - 1) as usize][x as usize] = Some(cumulative + d);
            //             open_points.insert((x, y - 1));
            //         }
            //     } else {
            //         self.cumulative[(y - 1) as usize][x as usize] = Some(cumulative + d);
            //         open_points.insert((x, y - 1));
            //     }
            // }

            if y < (height as i32 - 1) {
                if self.best[(y + 1) as usize][x as usize].is_none() {
                    let d = self.individual[(y + 1) as usize][x as usize];

                    if let Some(c) = self.cumulative[(y + 1) as usize][x as usize] {
                        if c > cumulative + d {
                            self.cumulative[(y + 1) as usize][x as usize] = Some(cumulative + d);
                            //  open_points.insert((x, y + 1));
                        }
                    } else {
                        self.cumulative[(y + 1) as usize][x as usize] = Some(cumulative + d);
                        //  open_points.insert((x, y + 1));
                    }
                }
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

        // Read the top-left bits into our map
        for line in input.lines() {
            individual.push(
                line.chars()
                    .map(|d| d.to_string().parse::<i32>().unwrap())
                    .collect(),
            );
            cumulative.push(vec![None; line.len()]);
        }

        // Seed the data
        cumulative[0][0] = Some(0);

        // Do the extra copies
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

        // Safety checks
        assert_eq!(5 * width, individual[0].len());
        assert_eq!(5 * height, individual.len());

        let best = cumulative.clone();

        let mut risks = Risks {
            individual,
            cumulative,
            best,
        };

        println!("{}", risks);

        // Do the path finding
        risks.walk();

        // Read answers
        let total1 = risks.best[height - 1][width - 1].unwrap();
        let total2 = risks.best[5 * height - 1][5 * width - 1].unwrap();

        (total1, total2)
    }
}

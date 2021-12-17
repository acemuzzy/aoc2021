pub struct Day17;

#[derive(Debug)]
struct Vector {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct TargetArea {
    top_left: Vector,
    bottom_right: Vector,
}

#[derive(Debug)]
struct Probe {
    location: Vector,
    velocity: Vector,
}

impl Probe {
    fn progress(&mut self) {
        self.location.x += self.velocity.x;
        self.location.y += self.velocity.y;
        self.velocity.y -= 1;
        self.velocity.x = std::cmp::max(self.velocity.x - 1, 0);
    }

    fn is_in_area(&self, target: &TargetArea) -> bool {
        self.location.x >= target.top_left.x
            && self.location.x <= target.bottom_right.x
            && self.location.y <= target.top_left.y
            && self.location.y >= target.bottom_right.y
    }

    fn hits(&mut self, target: &TargetArea) -> Option<i32> {
        let mut max_height = 0;

        while self.location.y >= target.bottom_right.y && self.location.x <= target.bottom_right.x {
            if self.location.y > max_height {
                max_height = self.location.y;
            }

            if self.is_in_area(target) {
                //println!("Success at {:?}!", self.location);
                return Some(max_height);
            }
            self.progress();
        }

        None
    }
}

impl crate::lib::DayInner<Day17, i32> for Day17 {
    fn day(&self) -> i32 {
        17
    }

    fn inner(&self, input: String) -> (i32, i32) {
        // Parse row of form "target area: x=20..30, y=-10..-5"
        let sections = input.split(' ').collect::<Vec<&str>>();
        let x_section = sections[2];
        println!("x_section {:?}", x_section);
        let y_section = sections[3];

        let x_range = x_section.split('=').collect::<Vec<&str>>()[1]
            .split("..")
            .collect::<Vec<&str>>();
        println!("x_range {:?}", x_range);
        let y_range = y_section.split('=').collect::<Vec<&str>>()[1]
            .split("..")
            .collect::<Vec<&str>>();

        let x0 = x_range[0].parse::<i32>().unwrap();
        let x1 = x_range[1][0..x_range[1].len() - 1].parse::<i32>().unwrap();
        let y0 = y_range[0].parse::<i32>().unwrap();
        let y1 = y_range[1].parse::<i32>().unwrap();

        assert!(x0 < x1);
        assert!(y0 < y1);

        let target = TargetArea {
            top_left: Vector { x: x0, y: y1 },
            bottom_right: Vector { x: x1, y: y0 },
        };
        println!("Target area: {:?}", target);

        // Scan hits, first looking at x-speed, then y-speed
        let mut max_heights: Vec<Option<i32>> = vec![];

        for x_speed in 1..=target.bottom_right.x {
            for y_speed in 0..10000 {
                let mut attempt = Probe {
                    location: Vector { x: 0, y: 0 },
                    velocity: Vector {
                        x: x_speed,
                        y: y_speed,
                    },
                };

                let result = attempt.hits(&target);

                if attempt.location.x > target.bottom_right.x
                    && attempt.location.y > target.top_left.y
                {
                    // Overshot entirely - no point trying higher angles
                    break;
                }

                if (attempt.location.x < target.top_left.x) && attempt.velocity.x == 0 {
                    // Fell short - no point trying higher angles
                    break;
                }

                if result.is_some() {
                    println!("{},{}", x_speed, y_speed);
                }

                max_heights.push(result);
            }

            for y_speed in 1..10000 {
                let y_speed = -y_speed;
                let mut attempt = Probe {
                    location: Vector { x: 0, y: 0 },
                    velocity: Vector {
                        x: x_speed,
                        y: y_speed,
                    },
                };

                let result = attempt.hits(&target);

                if attempt.location.x < target.top_left.x && attempt.velocity.x == 0 {
                    // Fell short - no point trying lower angles
                    break;
                }

                if attempt.location.x < target.top_left.x && attempt.location.y < target.top_left.y
                {
                    // Under entirely - no point trying lowing angles
                    break;
                }

                if result.is_some() {
                    println!("{},{}", x_speed, y_speed);
                }

                max_heights.push(result);
            }
        }

        let max_heights = max_heights.iter().flatten().collect::<Vec<&i32>>();

        (
            **max_heights.iter().max().unwrap(),
            max_heights.len() as i32,
        )
    }
}

use std::collections::HashMap;

pub struct Day12;

impl Day12 {
    fn is_small(name: &str) -> bool {
        let c: char = name.chars().collect::<Vec<char>>()[0];
        ('a'..='z').contains(&c)
    }

    fn walk1(
        leg_start: &str,
        map: &HashMap<&str, Vec<&str>>,
        mut visited_points: HashMap<String, i32>,
    ) -> i32 {
        let point = visited_points.get(&leg_start.to_string());
        match point {
            None => {
                visited_points.insert(leg_start.to_string(), 0);
            }
            Some(_) => {
                visited_points.insert(leg_start.to_string(), 1);
            }
        }

        if leg_start == "end" {
            return 1;
        }

        let d1 = map
            .get(leg_start)
            .unwrap()
            .iter()
            .map(|x| {
                if Self::is_small(x) && visited_points.get(&x.to_string()).is_some() {
                    0
                } else {
                    Self::walk1(x, map, visited_points.clone())
                }
            })
            .sum();

        d1
    }

    fn walk2(
        leg_start: &str,
        map: &HashMap<&str, Vec<&str>>,
        mut visited_points: HashMap<String, i32>,
        double: bool,
        mut route: Vec<String>,
    ) -> i32 {
        route.push(leg_start.to_string());
        let point = visited_points.get(&leg_start.to_string());
        match point {
            None => {
                visited_points.insert(leg_start.to_string(), 0);
            }
            Some(_) => {
                visited_points.insert(leg_start.to_string(), 1);
            }
        }

        if leg_start == "end" {
            return 1;
        }

        let d1 = map
            .get(leg_start)
            .unwrap()
            .iter()
            .filter(|x| **x != "start")
            .map(|x| {
                if Self::is_small(x) && visited_points.get(&x.to_string()).is_some() {
                    if double {
                        0
                    } else {
                        Self::walk2(x, map, visited_points.clone(), true, route.clone())
                    }
                } else {
                    Self::walk2(x, map, visited_points.clone(), double, route.clone())
                }
            })
            .sum();

        d1
    }

    // fn walk2(
    //     leg_start: &str,
    //     map: &HashMap<&str, Vec<&str>>,
    //     mut visited_points: HashMap<String, i32>,
    //     mut route: Vec<String>,
    // ) -> i32 {
    //     //println!("Trying path {:?}, from {}", visited_points, leg_start);
    //     route.push(leg_start.to_string());
    //     if leg_start == "end" {
    //         println!("Path {:?}", route.join(","));
    //         return 1;
    //     }

    //     //visited_points.push(leg_start.to_string());

    //     println!("Trying path {:?}, from {}, ", route, leg_start);

    //     let d2 = map
    //         .get(leg_start)
    //         .unwrap()
    //         .iter()
    //         .map(|x| {
    //             let entry = visited_points.get(&x.to_string());

    //             if Self::is_small(x) && visited_points.values().collect::<Vec<&i32>>().contains(&&2) {
    //                 0
    //             } else if *x == "start" {
    //                 0
    //             } else {
    //                 if Self::is_small(x) {
    //                     let point = visited_points.get(&leg_start.to_string());
    //                     match point {
    //                         None => {
    //                             visited_points.insert(leg_start.to_string(), 1);
    //                         }
    //                         Some(1) => {
    //                             visited_points.insert(leg_start.to_string(), 2);
    //                         }
    //                         _ => {
    //                             assert_eq!(Self::is_small(leg_start), false);
    //                         }
    //                     }
    //                 }
    //                 Self::walk2(x, map, visited_points.clone(), route.clone())
    //             }
    //         })
    //         .sum();

    //     d2
    // }
}

impl crate::lib::DayInner<Day12, i32> for Day12 {
    fn day(&self) -> i32 {
        12
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let lines: Vec<&str> = input.lines().collect();
        let mut map = HashMap::<&str, Vec<&str>>::new();

        for line in lines {
            let split: Vec<&str> = line.split('-').collect();
            let (start, end) = (split[0], split[1]);
            let entry = map.entry(start).or_insert_with(Vec::new);
            entry.push(end);
            let entry = map.entry(end).or_insert_with(Vec::new);
            entry.push(start);
        }

        let routes1 = Self::walk1("start", &map, HashMap::new());
        let routes2 = Self::walk2("start", &map, HashMap::new(), false, vec![]);

        (routes1, routes2)
    }
}

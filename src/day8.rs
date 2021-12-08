use std::collections::{HashMap, HashSet};

pub struct Day8;

#[derive(Debug)]
struct Row {
    #[allow(dead_code)]
    patterns: Vec<HashSet<char>>,
    digits: Vec<HashSet<char>>,
}

impl crate::lib::DayInner<Day8, i32> for Day8 {
    fn day(&self) -> i32 {
        8
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let lines: Vec<&str> = input.lines().collect();
        let mut rows: Vec<Row> = vec![];

        for line in lines {
            let halves: Vec<&str> = line.split(" | ").collect();
            rows.push(Row {
                patterns: halves[0].split(' ').map(|s| s.chars().collect()).collect(),
                digits: halves[1].split(' ').map(|s| s.chars().collect()).collect(),
            })
        }

        let mut distinct_count = 0;
        let mut sum = 0;

        for row in rows.iter() {
            // Do the count for part 1
            for digit in row.digits.iter() {
                match digit.len() {
                    // 1 -> 2, 7 -> 3, 4 -> 4, 8 -> 7
                    2 | 3 | 4 | 7 => {
                        distinct_count += 1;
                    }
                    _ => {}
                }
            }

            // Do the pattern analysis for part 2
            let mut len_map = HashMap::<usize, Vec<HashSet<char>>>::new();
            let mut digit_map = HashMap::<i32, HashSet<char>>::new();
            for pattern in row.patterns.iter() {
                let l = pattern.len();
                let vec = len_map.entry(l).or_insert_with(Vec::new);
                vec.push(pattern.clone());
            }

            // Do direct inferences
            digit_map.entry(1).or_insert_with(|| len_map[&2][0].clone());
            digit_map.entry(7).or_insert_with(|| len_map[&3][0].clone());
            digit_map.entry(4).or_insert_with(|| len_map[&4][0].clone());
            digit_map.entry(8).or_insert_with(|| len_map[&7][0].clone());

            // 9 is the only 6-segment number containing 4
            let four = digit_map.get(&4).unwrap().clone();
            'outer9: for s in len_map.get(&6).unwrap().iter() {
                for c in four.iter() {
                    if !s.contains(c) {
                        continue 'outer9;
                    }
                }
                digit_map.entry(9).or_insert_with(|| s.clone());
            }

            // 2 is the only 5-segment contained in 9
            let nine = digit_map.get(&9).unwrap().clone();
            for s in len_map.get(&5).unwrap().iter() {
                for c in s.iter() {
                    if !nine.contains(c) {
                        digit_map.entry(2).or_insert_with(|| s.clone());
                        break;
                    }
                }
            }
            let two = digit_map.get(&2).unwrap().clone();

            // 3 is the only 5-segment containing 7
            let seven = digit_map.get(&7).unwrap().clone();
            'outer3: for s in len_map.get(&5).unwrap().iter() {
                for c in seven.iter() {
                    if !s.contains(c) {
                        continue 'outer3;
                    }
                }
                digit_map.entry(3).or_insert_with(|| s.clone());
            }
            let three = digit_map.get(&3).unwrap().clone();

            // 5 is the only other 5-segment
            for s in len_map.get(&5).unwrap().iter() {
                if s != &two && s != &three {
                    digit_map.entry(5).or_insert_with(|| s.clone());
                }
            }
            let five = digit_map.get(&5).unwrap().clone();

            // 0 is the only 6-segment which does not contain 5
            for s in len_map.get(&6).unwrap().iter() {
                for c in five.iter() {
                    if !s.contains(c) {
                        digit_map.entry(0).or_insert_with(|| s.clone());
                        break;
                    }
                }
            }
            let zero = digit_map.get(&0).unwrap().clone();

            // 6 is the only other 6-segment
            for s in len_map.get(&6).unwrap().iter() {
                if s != &zero && s != &nine {
                    digit_map.entry(6).or_insert_with(|| s.clone());
                }
            }

            // Right, we finally have our digit set.  We can now finally work out the number, and increment or sum.
            let mut val = 1000;
            let mut delta = 0;
            for ii in 0..4 {
                for (k, v) in digit_map.iter() {
                    if &row.digits[ii] == v {
                        delta += val * k;
                        break;
                    }
                }
                val /= 10;
            }

            sum += delta;
        }

        (distinct_count, sum)
    }
}

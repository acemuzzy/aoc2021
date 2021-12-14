use std::collections::HashMap;

pub struct Day14;

impl Day14 {
    fn gen_string(
        map: &HashMap<(char, char), char>,
        current: HashMap<(char, char), i64>,
    ) -> HashMap<(char, char), i64> {
        let mut output = HashMap::<(char, char), i64>::new();

        for (k, v) in current.iter() {
            let (c1, c2) = k;
            let lookup = map.get(&(*c1, *c2));
            if let Some(c) = lookup {
                *output.entry((*c1, *c)).or_insert(0) += v;
                *output.entry((*c, *c2)).or_insert(0) += v;
            } else {
                *output.entry((*c1, *c2)).or_insert(0) += v;
            }
        }

        output
    }
}

impl crate::lib::DayInner<Day14, i64> for Day14 {
    fn day(&self) -> i32 {
        14
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let lines: Vec<&str> = input.lines().collect();
        let mut map = HashMap::<(char, char), char>::new();

        let mut current = HashMap::<(char, char), i64>::new();
        let chars = lines[0].chars().collect::<Vec<char>>();
        let first = chars[0];
        let last = chars[chars.len() - 1];
        for ii in 0..chars.len() - 1 {
            let c1 = chars[ii];
            let c2 = chars[ii + 1];
            *current.entry((c1, c2)).or_insert(0) += 1;
        }

        for line in &lines[2..] {
            let chars = line.chars().collect::<Vec<char>>();
            map.insert((chars[0], chars[1]), chars[6]);
        }

        for _ in 0..10 {
            current = Day14::gen_string(&map, current);
        }

        let mut counts = HashMap::<char, i64>::new();
        for ((c1, c2), v) in current.iter() {
            *counts.entry(*c1).or_insert(0) += v;
            *counts.entry(*c2).or_insert(0) += v;
        }
        *counts.entry(first).or_insert(0) += 1;
        *counts.entry(last).or_insert(0) += 1;

        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();
        let p1 = (max - min) / 2;

        for _ in 0..30 {
            current = Day14::gen_string(&map, current);
        }

        let mut counts = HashMap::<char, i64>::new();
        for ((c1, c2), v) in current.iter() {
            *counts.entry(*c1).or_insert(0) += v;
            *counts.entry(*c2).or_insert(0) += v;
        }
        *counts.entry(first).or_insert(0) += 1;
        *counts.entry(last).or_insert(0) += 1;

        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();
        let p2 = (max - min) / 2;

        (p1, p2)
    }
}

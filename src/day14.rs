use std::collections::HashMap;

pub struct Day14;

impl Day14 {
    fn gen_string(map: &HashMap<(char, char), char>, current: String) -> String {
        let chars = current.chars().collect::<Vec<char>>();
        let mut output: Vec<char> = vec![];
        for ii in 0..chars.len() - 1 {
            let c1 = chars[ii];
            let c2 = chars[ii + 1];
            let lookup = map.get(&(c1, c2));
            output.push(c1);
            if let Some(c) = lookup {
                output.push(*c);
            }
        }
        output.push(chars[chars.len() - 1]);
        output.iter().collect()
    }
}

impl crate::lib::DayInner<Day14, i32> for Day14 {
    fn day(&self) -> i32 {
        14
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let lines: Vec<&str> = input.lines().collect();
        let mut map = HashMap::<(char, char), char>::new();

        let mut current = lines[0].to_string();

        for line in &lines[2..] {
            let chars = line.chars().collect::<Vec<char>>();
            map.insert((chars[0], chars[1]), chars[6]);
        }

        println!("Map {:?}", map);

        for _ in 0..10 {
            current = Day14::gen_string(&map, current);
        }

        let mut counts = HashMap::<char, i32>::new();
        for c in current.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();

        (max - min, 0)
    }
}

use std::collections::HashMap;

pub struct Day10;

struct ClosePair {
    opener: char,
    value: i64,
}
impl crate::lib::DayInner<Day10, i64> for Day10 {
    fn day(&self) -> i32 {
        10
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let lines: Vec<&str> = input.lines().collect();

        let mut close_map = HashMap::<char, ClosePair>::new();
        close_map.insert(
            ')',
            ClosePair {
                opener: '(',
                value: 3,
            },
        );
        close_map.insert(
            ']',
            ClosePair {
                opener: '[',
                value: 57,
            },
        );
        close_map.insert(
            '}',
            ClosePair {
                opener: '{',
                value: 1197,
            },
        );
        close_map.insert(
            '>',
            ClosePair {
                opener: '<',
                value: 25137,
            },
        );

        let mut sum1 = 0i64;
        let mut unwrap_set: Vec<i64> = vec![];
        for line in lines {
            let mut queue: Vec<char> = vec![];
            let mut incomplete = true;
            for c in line.chars() {
                match c {
                    '[' | '<' | '{' | '(' => {
                        queue.push(c);
                    }
                    ']' | '>' | '}' | ')' => {
                        let opener = close_map.get(&c).unwrap();
                        if Some(opener.opener) != queue.pop() {
                            sum1 += opener.value;
                            incomplete = false;
                            break;
                        }
                    }
                    _ => {
                        panic!("Unexpected character {}", c);
                    }
                }
            }

            // Handle completion if applicable
            if incomplete {
                let mut calc = 0;
                let mut unwrap_queue: Vec<char> = vec![];
                while !queue.is_empty() {
                    let c = queue.pop().unwrap();
                    if close_map.keys().any(|x| x == &c) {
                        unwrap_queue.push(c);
                        continue;
                    }
                    if unwrap_queue.pop().is_some() {
                        continue;
                    }
                    calc *= 5;
                    calc += match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => {
                            panic!("Unexpected unwrap character!")
                        }
                    }
                }

                unwrap_set.push(calc);
            }
        }

        unwrap_set.sort_unstable();

        (sum1, unwrap_set[(unwrap_set.len() - 1) / 2])
    }
}

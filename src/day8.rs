pub struct Day8;

struct Row<'a> {
    #[allow(dead_code)]
    patterns: Vec<&'a str>,
    digits: Vec<&'a str>,
}

impl crate::lib::DayInner<Day8, i32> for Day8 {
    fn day(&self) -> i32 {
        8
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let lines: Vec<&str> = input.lines().collect();
        let mut rows: Vec<Row> = vec![];

        for line in lines {
            let halves: Vec<&str> = line.split('|').collect();
            rows.push(Row {
                patterns: halves[0].split(' ').collect(),
                digits: halves[1].split(' ').collect(),
            })
        }

        let mut distinct_count = 0;
        for row in rows.iter() {
            for digit in row.digits.iter() {
                match digit.len() {
                    // 1 -> 2, 7 -> 3, 4 -> 4, 8 -> 7
                    2 | 3 | 4 | 7 => {
                        distinct_count += 1;
                    }
                    _ => {}
                }
            }
        }

        (distinct_count, 0)
    }
}

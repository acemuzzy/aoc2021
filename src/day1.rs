pub struct Day1;

impl crate::lib::DayInner<Day1> for Day1 {
    fn day(&self) -> i32 {
        1
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let lines: Vec<&str> = input.lines().collect();
        let int_lines: Vec<i32> = lines.iter().map(|x| x.parse::<i32>().unwrap()).collect();

        let mut last_depths: [Option<i32>; 3] = [None, None, None];

        let mut counter1 = 0;
        let mut counter2 = 0;

        for int_line in int_lines {
            if let Some(d) = last_depths[0] {
                if d < int_line {
                    counter1 += 1;
                }
            }

            if let Some(d) = last_depths[2] {
                if d < int_line {
                    counter2 += 1;
                }
            }

            last_depths = [Some(int_line), last_depths[0], last_depths[1]]
        }

        println!("Counter1 is {}", counter1);
        println!("Counter2 is {}", counter2);

        (counter1, counter2)
    }
}

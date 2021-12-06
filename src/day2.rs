pub struct Day2;

impl crate::lib::DayInner<Day2, i32> for Day2 {
    fn day(&self) -> i32 {
        2
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let lines: Vec<&str> = input.lines().collect();
        let mut pos1: (i32, i32) = (0, 0);
        let mut pos2: (i32, i32) = (0, 0);
        let mut aim: i32 = 0;

        for line in lines {
            let split: Vec<&str> = line.split(' ').collect();

            let shift: (i32, i32) = match (split[0], split[1]) {
                ("forward", x) => (x.parse().unwrap(), 0),
                ("up", y) => (0, -y.parse::<i32>().unwrap()),
                ("down", y) => (0, y.parse().unwrap()),
                _ => {
                    panic!("Huh??");
                }
            };

            pos1 = (pos1.0 + shift.0, pos1.1 + shift.1);
            aim += shift.1;
            pos2 = (pos2.0 + shift.0, pos2.1 + aim * shift.0);
        }

        println!("Pos1 is {:?}, product {}", pos1, pos1.0 * pos1.1);
        println!("Pos2 is {:?}, product {}", pos2, pos2.0 * pos2.1);

        (pos1.0 * pos1.1, pos2.0 * pos2.1)
    }
}

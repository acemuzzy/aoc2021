pub struct Day3;

impl crate::lib::DayInner<Day3, i32> for Day3 {
    fn day(&self) -> i32 {
        3
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let lines: Vec<&str> = input.lines().collect();

        let width = lines[0].len();
        let number = lines.len();
        let mut counts: Vec<i32> = vec![0; width];

        for line in lines.iter() {
            for x in 0..width {
                counts[x] += &line[x..x + 1].parse().unwrap();
            }
        }

        let mut gammavec: Vec<i32> = vec![];
        let mut gamma: i32 = 0;
        let mut epsilon: i32 = 0;
        let mut val = 2i32.pow(width as u32);
        for count in &counts {
            val /= 2;
            if count >= &(number as i32 - count) {
                gammavec.push(1);
                gamma += val;
            } else {
                gammavec.push(0);
                epsilon += val;
            }
        }

        println!("Gammavec is {:?}", gammavec);
        println!("Gamma is {}", gamma);
        println!("Epsilon is {}", epsilon);
        let power = gamma * epsilon;
        println!("Power is {}", power);

        let mut oxygen: Vec<&str> = lines.clone();

        for index in 0..width {
            let mut count = 0;
            for line in oxygen.iter() {
                count += &line[index..index + 1].parse().unwrap();
            }
            let target = if count >= (oxygen.len() - count) {
                '1'
            } else {
                '0'
            };

            oxygen = oxygen
                .into_iter()
                .filter(|l| {
                    let filter = l.chars().collect::<Vec<char>>()[index as usize] == target;
                    filter
                })
                .collect();

            if oxygen.len() == 1 {
                println!("Got ox {:?}", oxygen);
                break;
            }
        }

        let mut carbon: Vec<&str> = lines;

        for index in 0..width {
            let mut count = 0;
            for line in carbon.iter() {
                count += &line[index..index + 1].parse().unwrap();
            }
            let target = if count >= (carbon.len() - count) {
                '0'
            } else {
                '1'
            };

            carbon = carbon
                .into_iter()
                .filter(|l| {
                    let filter = l.chars().collect::<Vec<char>>()[index as usize] == target;
                    filter
                })
                .collect();

            if carbon.len() == 1 {
                println!("Got carb {:?}", carbon);
                break;
            }
        }

        let mut ox = 0;
        let mut val = 2i32.pow(width as u32);
        for o in oxygen[0].chars() {
            val /= 2;
            if o == '1' {
                ox += val;
            }
        }
        println!("Ox {}", ox);

        let mut carb = 0;
        let mut val = 2i32.pow(width as u32);
        for c in carbon[0].chars() {
            val /= 2;
            if c == '1' {
                carb += val;
            }
        }
        println!("Carb {}", carb);

        let lsr = ox * carb;
        println!("Life support rating: {}", lsr);

        (power, lsr)
    }
}

pub struct Day6;

fn next_generation(pots: Vec<i64>) -> Vec<i64> {
    let mut new_pots: Vec<i64> = vec![0; 9];

    new_pots[..8].clone_from_slice(&pots[1..9]);
    new_pots[6] += pots[0];
    new_pots[8] = pots[0];

    new_pots
}

impl crate::lib::DayInner<Day6, i64> for Day6 {
    fn day(&self) -> i32 {
        6
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let seeds: Vec<_> = input
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        println!("Seeds {:?}", seeds);

        let mut pots = vec![0; 9];
        for seed in seeds {
            pots[seed as usize] += 1;
        }

        for _ in 0..80 {
            pots = next_generation(pots);
        }

        let size1 = pots.iter().sum::<i64>();

        for _ in 0..176 {
            pots = next_generation(pots);
        }

        let size2 = pots.iter().sum::<i64>();

        (size1, size2)
    }
}

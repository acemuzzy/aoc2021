pub struct Day7;

impl crate::lib::DayInner<Day7, i32> for Day7 {
    fn day(&self) -> i32 {
        7
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let seeds: Vec<_> = input
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let min = seeds.iter().min().unwrap();
        let max = seeds.iter().max().unwrap();
        let mut fuels1 = vec![];
        let mut fuels2 = vec![];

        for ii in *min..*max {
            fuels1.push(seeds.iter().map(|x| (x - ii).abs()).sum());
            fuels2.push(
                seeds
                    .iter()
                    .map(|x| {
                        let d = (x - ii).abs();
                        (d * (d + 1)) / 2
                    })
                    .sum(),
            );
        }
        let fuel1 = fuels1.iter().min().unwrap();
        let fuel2 = fuels2.iter().min().unwrap();

        (*fuel1, *fuel2)
    }
}

use std::fs;

pub trait DayInner<T> {
    fn inner(&self, input: String) -> (i32, i32);
    fn day(&self) -> i32;
}

pub trait Day<T>
where
    T: DayInner<T>,
{
    fn run(&self);
    fn test(&self) -> (i32, i32);
}

impl<T> Day<T> for T
where
    T: DayInner<T>,
{
    fn run(&self) {
        let input = get_file_content(self.day());
        self.inner(input);
    }

    fn test(&self) -> (i32, i32) {
        let input = get_test_content(self.day());
        self.inner(input)
    }
}

fn get_file_content(day: i32) -> String {
    let filename = format!("inputs/day{}.input", day);
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn get_test_content(day: i32) -> String {
    let filename = format!("inputs/day{}.test", day);
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

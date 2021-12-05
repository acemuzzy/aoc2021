use std::fs;
use std::path::PathBuf;

const TEST_SUFFIX: &str = "test";
const INPUT_SUFFIX: &str = "input";

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
        let input = get_input_content(self.day(), false);
        self.inner(input);
    }

    fn test(&self) -> (i32, i32) {
        let input = get_input_content(self.day(), true);
        self.inner(input)
    }
}

fn get_file_type(test: bool) -> &'static str {
    if test {
        TEST_SUFFIX
    } else {
        INPUT_SUFFIX
    }
}

fn get_file_name(day: i32, test: bool) -> PathBuf {
    let mut path = PathBuf::new();
    path.push("inputs");
    path.push(format!("day{}", day));
    path.set_extension(get_file_type(test));
    path
}

fn get_file_content(filename: PathBuf) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn get_input_content(day: i32, test: bool) -> String {
    get_file_content(get_file_name(day, test))
}

mod day1;
mod day2;
mod day3;
mod day4;
mod lib;

use crate::lib::Day;

fn main() {
    // day1::Day1 {}.run();
    // day2::Day2 {}.run();
    // day3::Day3 {}.run();
    day4::Day4 {}.run();
}

#[cfg(test)]
mod test {
    use super::{day1, day2, day3, day4, lib::Day};

    #[test]
    fn day1() {
        assert_eq!(day1::Day1 {}.test(), (7, 5));
    }

    #[test]
    fn day2() {
        assert_eq!(day2::Day2 {}.test(), (150, 900));
    }

    #[test]
    fn day3() {
        assert_eq!(day3::Day3 {}.test(), (198, 230));
    }

    #[test]
    fn day4() {
        assert_eq!(day4::Day4 {}.test(), (4512, 1924));
    }
}

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod lib;

use crate::lib::Day;

fn main() {
    // day1::Day1 {}.run();
    // day2::Day2 {}.run();
    // day3::Day3 {}.run();
    // day4::Day4 {}.run();
    // day5::Day5 {}.run();
    // day6::Day6 {}.run();
    // day7::Day7 {}.run();
    // day8::Day8 {}.run();
    // day9::Day9 {}.run();
    day10::Day10 {}.run();
}

#[cfg(test)]
mod test {
    use super::{day1, day10, day2, day3, day4, day5, day6, day7, day8, day9, lib::Day};

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

    #[test]
    fn day5() {
        assert_eq!(day5::Day5 {}.test(), (5, 12));
    }

    #[test]
    fn day6() {
        assert_eq!(day6::Day6 {}.test(), (5934, 26984457539));
    }

    #[test]
    fn day7() {
        assert_eq!(day7::Day7 {}.test(), (37, 168));
    }

    #[test]
    fn day8() {
        assert_eq!(day8::Day8 {}.test(), (26, 61229));
    }

    #[test]
    fn day9() {
        assert_eq!(day9::Day9 {}.test(), (15, 1134));
    }

    #[test]
    fn day10() {
        assert_eq!(day10::Day10 {}.test(), (26397, 288957));
    }
}

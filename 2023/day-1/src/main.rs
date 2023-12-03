use std::{iter::Rev, str::Chars, time::Instant};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let start_part_1 = Instant::now();
    let part_1_result = solution_part_1(&input);
    let part_1_time = start_part_1.elapsed();

    println!("Part 1: {} ({:?})", part_1_result, part_1_time);

    let input = std::fs::read_to_string("input.txt").unwrap();

    let start_part_2 = Instant::now();
    let part_2_result = solution_part_2(&input);
    let part_2_time = start_part_2.elapsed();

    println!("Part 2: {} ({:?})", part_2_result, part_2_time);
}

fn solution_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find(|c| c.is_numeric()).unwrap();
            let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();

            format!("{}{}", first, last).parse::<usize>().unwrap()
        })
        .sum()
}

trait DigitsSearcher {
    fn find_first_digit(self) -> Option<u32>;
}

impl<I> DigitsSearcher for I
where
    I: Iterator<Item = char>,
{
    fn find_first_digit(mut self) -> Option<u32> {
        let mut buff = String::new();

        self.find_map(|c| {
            if c.is_numeric() {
                Some(c.to_digit(10).unwrap())
            } else {
                buff.push(c);

                if let Some(digit) = buff.find_spelled_out_digit() {
                    Some(digit)
                } else {
                    None
                }
            }
        })
    }
}

trait IsDigitString {
    fn find_spelled_out_digit(&self) -> Option<u32>;
}

impl IsDigitString for String {
    fn find_spelled_out_digit(&self) -> Option<u32> {
        if self.ends_with(&"one") || self.starts_with(&"eno") {
            return Some(1);
        } else if self.ends_with(&"two") || self.starts_with(&"owt") {
            return Some(2);
        } else if self.ends_with(&"three") || self.starts_with(&"eerht") {
            return Some(3);
        } else if self.ends_with(&"four") || self.starts_with(&"ruof") {
            return Some(4);
        } else if self.ends_with(&"five") || self.starts_with(&"evif") {
            return Some(5);
        } else if self.ends_with(&"six") || self.starts_with(&"xis") {
            return Some(6);
        } else if self.ends_with(&"seven") || self.starts_with(&"neves") {
            return Some(7);
        } else if self.ends_with(&"eight") || self.starts_with(&"thgie") {
            return Some(8);
        } else if self.ends_with(&"nine") || self.starts_with(&"enin") {
            return Some(9);
        } else {
            return None;
        }
    }
}

fn solution_part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find_first_digit();

            let last = line.chars().rev().find_first_digit();

            format!("{}{}", first.unwrap(), last.unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
        "#
        .trim();

        assert_eq!(solution_part_1(input), 142);
    }

    #[test]
    fn part2() {
        let input = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen 
        "#
        .trim();

        assert_eq!(solution_part_2(input), 281);
    }
}

use std::time::Instant;

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

fn solution_part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut forwards = line.chars().into_iter();
            let mut backwards = line.chars().rev().into_iter();

            let mut first: Option<char> = None;
            let mut last: Option<char> = None;

            loop {
                if first.is_none() {
                    if let Some(forward_next) = forwards.next() {
                        if forward_next.is_numeric() {
                            first = Some(forward_next);
                        }
                    }
                }

                if last.is_none() {
                    if let Some(backwards_next) = backwards.next() {
                        if backwards_next.is_numeric() {
                            last = Some(backwards_next);
                        }
                    }
                }

                if first.is_some() && last.is_some() {
                    break;
                }
            }

            format!("{}{}", first.unwrap(), last.unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn solution_part_2(input: &str) -> i32 {
    let dict = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let inverse_dict = [
        "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];

    input
        .lines()
        .map(|line| {
            let mut forwards = line.chars().into_iter();
            let mut backwards = line.chars().rev().into_iter();

            let mut buff_first = String::new();
            let mut buff_last = String::new();

            let mut first: Option<char> = None;
            let mut last: Option<char> = None;

            loop {
                if first.is_none() {
                    if let Some(forward_next) = forwards.next() {
                        if forward_next.is_numeric() {
                            first = Some(forward_next);
                        } else {
                            buff_first.push(forward_next);

                            if buff_first.len() >= 3 {
                                if let Some(num) = dict.iter().position(|x| buff_first.contains(x))
                                {
                                    first = Some(char::from_digit((num + 1) as u32, 10).unwrap());
                                }
                            }
                        }
                    }
                }

                if last.is_none() {
                    if let Some(backwards_next) = backwards.next() {
                        if backwards_next.is_numeric() {
                            last = Some(backwards_next);
                        } else {
                            buff_last.push(backwards_next);

                            if buff_last.len() >= 3 {
                                if let Some(num) =
                                    inverse_dict.iter().position(|x| buff_last.contains(x))
                                {
                                    last = Some(char::from_digit((num + 1) as u32, 10).unwrap());
                                }
                            }
                        }
                    }
                }

                if first.is_some() && last.is_some() {
                    break;
                }
            }

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

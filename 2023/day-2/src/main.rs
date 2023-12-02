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

#[derive(Debug)]
struct Balls {
    blue: i32,
    red: i32,
    green: i32,
}

impl Balls {
    fn fit_in_constraint(&self, constraints: &Balls) -> bool {
        self.blue <= constraints.blue
            && self.red <= constraints.red
            && self.green <= constraints.green
    }
}

fn solution_part_1(input: &str) -> i32 {
    let constrained_balls = Balls {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut answer: i32 = 0;

    input.lines().for_each(|line| {
        let index_of_first_space = line.find(" ");

        let mut iter_after_space = line
            .chars()
            .skip(index_of_first_space.unwrap() + 1)
            .into_iter();

        let mut game_id = String::new();

        loop {
            let c = iter_after_space.next().unwrap();

            if c.eq(&':') {
                break;
            }

            game_id.push(c);
        }

        let game_id = game_id.parse::<i32>().unwrap();
        let mut balls = Balls {
            blue: 0,
            red: 0,
            green: 0,
        };

        let mut round_state = String::new();
        let mut is_game_valid = true;

        loop {
            let next = iter_after_space.next();

            if let None = next {
                break;
            }

            let next = next.unwrap();

            if next.is_digit(10) {
                round_state.push(next);
            }

            if next.eq(&'b') && round_state.len() > 0 {
                balls.blue = round_state.parse::<i32>().unwrap();

                round_state.clear();
            }

            if next.eq(&'r') && round_state.len() > 0 {
                balls.red = round_state.parse::<i32>().unwrap();

                round_state.clear();
            }

            if next.eq(&'g') && round_state.len() > 0 {
                balls.green = round_state.parse::<i32>().unwrap();

                round_state.clear();
            }

            if next.eq(&';') {
                round_state.clear();

                if !balls.fit_in_constraint(&constrained_balls) {
                    is_game_valid = false;

                    break;
                }
            }
        }

        if !balls.fit_in_constraint(&constrained_balls) {
            is_game_valid = false;
        }

        if is_game_valid {
            answer += game_id;
        }
    });

    answer
}

fn solution_part_2(input: &str) -> i32 {
    let mut answer: i32 = 0;

    input.lines().for_each(|line| {
        let index_of_first_space = line.find(" ");

        let mut iter_after_space = line
            .chars()
            .skip(index_of_first_space.unwrap() + 1)
            .into_iter();

        let mut game_id = String::new();

        loop {
            let c = iter_after_space.next().unwrap();

            if c.eq(&':') {
                break;
            }

            game_id.push(c);
        }

        let mut balls = Balls {
            blue: 0,
            red: 0,
            green: 0,
        };

        let mut round_state = String::new();

        loop {
            let next = iter_after_space.next();

            if let None = next {
                break;
            }

            let next = next.unwrap();

            if next.is_digit(10) {
                round_state.push(next);
            }

            if next.eq(&'b') && round_state.len() > 0 {
                let new_blue = round_state.parse::<i32>().unwrap();

                if balls.blue < new_blue {
                    balls.blue = new_blue;
                }

                round_state.clear();
            }

            if next.eq(&'r') && round_state.len() > 0 {
                let new_red = round_state.parse::<i32>().unwrap();

                if balls.red < new_red {
                    balls.red = new_red;
                }

                round_state.clear();
            }

            if next.eq(&'g') && round_state.len() > 0 {
                let new_green = round_state.parse::<i32>().unwrap();

                if balls.green < new_green {
                    balls.green = new_green;
                }

                round_state.clear();
            }
        }

        answer += balls.blue * balls.red * balls.green;
    });

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#
        .trim();

        assert_eq!(solution_part_1(input), 8);
    }

    #[test]
    fn part1_real() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(solution_part_1(&input), 2563);
    }

    #[test]
    fn part2() {
        let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#
        .trim();

        assert_eq!(solution_part_2(input), 2286);
    }
}

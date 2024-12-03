

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|x| x.split(' ').map(|t| t.parse().unwrap()).collect()).collect()
}

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    Decreasing,
    Increasing,
    Initial
}

#[derive(Debug)]
struct ChangeError {
    previous_direction: Direction,
    level_difference: i32,
    index: usize
}

#[aoc(day2, part1)]
pub fn part_1(input: &Vec<Vec<i32>>) -> u32 {

    let mut count_good = 0;

    for report in input {
        let mut direction = Direction::Initial;
        let mut change_errors = Vec::new();
        for i in 1..report.len() {

            match check_for_error(i-1, i, report, &direction) {
                Ok(new_direciton) => { direction = new_direciton }
                Err(error) => { change_errors.push(error); }
            }

        }
        if change_errors.len() == 0 {
            count_good += 1;
        }

    }

    count_good as u32
}

#[aoc(day2, part2)]
pub fn part_2(input: &Vec<Vec<i32>>) -> u32 {

    let mut count_good = 0;

    let mut report_count = 0;
    println!("report_count!report!change_errors!status");
    for report in input {
        let mut direction = Direction::Initial;
        let mut change_errors = Vec::new();
        for i in 1..report.len() {

            match check_for_error(i-1, i, report, &direction) {
                Ok(new_direciton) => { direction = new_direciton }
                Err(error) => { change_errors.push(error); }
            }

        }
        print!("{}!{:?}!{:?}!", report_count, report, change_errors);
        if change_errors.len() == 0 {
            print!("None");
            count_good += 1;
        } else {
            if change_errors.len() == 1 {
                let error = &change_errors[0];
                if error.index != report.len() - 1 {

                    if let Ok(_) = check_for_error(error.index-1, error.index + 1, report, &error.previous_direction) {
                        print!("good");
                        count_good += 1;
                    } else if error.index == 1 {
                        if let Ok(_) = check_for_error(error.index, error.index + 1, report, &Direction::Initial) {
                            print!("good_initial");
                            count_good += 1;
                        } else {
                            print!("bad_initial");
                        }
                    } else {
                        print!("bad");
                    }
                } else {
                    // if we're here then we've only one error and it's the last field so it can just be deleted
                    print!("last");
                    count_good += 1;
                }
            } else {
                print!(">1");
            }
        }
        println!();

        report_count += 1;

    }

    count_good as u32
}

fn check_for_error(index_prev: usize, index: usize, report: &Vec<i32>, previous_direction: &Direction) -> Result<Direction, ChangeError> {
    let a = report[index_prev];
    let b = report[index];

    let level_difference = (b - a).abs();

    let mut direction = Direction::Initial;
    if a < b {
        if !matches!(previous_direction, Direction::Initial) && matches!(previous_direction, Direction::Decreasing) {
            return Err(ChangeError { index, previous_direction: previous_direction.clone(), level_difference });
        }
        direction = Direction::Increasing;
    } else if a > b {
        if !matches!(previous_direction, Direction::Initial) && matches!(previous_direction, Direction::Increasing) {
            return Err(ChangeError { index, previous_direction: previous_direction.clone(), level_difference });
        }
        direction = Direction::Decreasing;
    } else if a == b {
        return Err(ChangeError { index, previous_direction: previous_direction.clone(), level_difference });
    }



    if level_difference < 1 || level_difference > 3 {
        return Err(ChangeError { index, previous_direction: previous_direction.clone(), level_difference });
    }

    Ok(direction)
}



#[cfg(test)]
mod tests {
    use super::{input_generator, part_1, part_2};

    #[test]
    fn test_input_generator() {
        let input = "1 2\n7 8 9";

        let output = input_generator(&input);

        assert_eq!(output.len(), 2);
        assert_eq!(output[0].len(), 2);
        assert_eq!(output[0][0], 1);
        assert_eq!(output[0][1], 2);
        assert_eq!(output[1].len(), 3);
        assert_eq!(output[1][0], 7);
        assert_eq!(output[1][1], 8);
        assert_eq!(output[1][2], 9);
    }

    #[test]
    fn test_part_1() {
        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1,2,7,8,9],
            vec![9,7,6,2,1],
            vec![1,3,2,4,5],
            vec![8,6,4,4,1],
            vec![1,3,6,7,9]
        ];

        let result = part_1(
            &input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_2() {
        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1,2,7,8,9],
            vec![9,7,6,2,1],
            vec![1,3,2,4,5],
            vec![8,6,4,4,1],
            vec![1,3,6,7,9]
        ];

        let result = part_2(
            &input);

        assert_eq!(result, 4);
    }
}
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Log the input
    let mut result: u32 = 0;

    // On each line of the input the desired value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
    for line in input.lines() {
        // Convert the line to a vector of digits
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        // Combine the first digit and the last digit (in that order) to form a single two-digit number.
        let first = digits[0];
        let last = digits[digits.len() - 1];
        let number = first * 10 + last;
        // Log the number
        result += number;
    }
    return Some(result);
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

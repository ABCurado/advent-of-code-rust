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

pub fn part_two(input: &str) -> Option<u32> {
    // On each line of the input the desired value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
    // some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
    // What is the sum of all of the input values?
    let mut result: u32 = 0;
    for line in input.lines() {
        // Convert the line to a string
        let mut line_chars: Vec<char> = line.chars().collect();
        // CAst line as string
        let mut line_string:String = line_chars.clone().into_iter().collect();
        // Replace the single digit number words with the respective digits
        let num_words = ["zero","one","two","three","four","five","six","seven","eight","nine"];
        for i in 0..10 { // makes the 2nd letter of each digit word into the digit itself
            let mut loc = line.find(num_words[i]);
            while loc.is_some() {
                line_chars[loc.unwrap() + 1] = char::from_digit(i as u32, 10).unwrap();
                line_string = line_chars.clone().into_iter().collect();
                loc = line_string.find(num_words[i])
            }
        }
        // println!("{}", line);
        let digits: Vec<u32> = line_string.chars().filter_map(|c| c.to_digit(10)).collect();
        // println!("{:?}", digits);
        // Combine the first digit and the last digit (in that order) to form a single two-digit number.
        let first = digits[0];
        let last = digits[digits.len() - 1];
        let number = first * 10 + last;
        // Log the number
        result += number;
    }
    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}

advent_of_code::solution!(2);
// Copied from https://github.com/danielhuang/aoc-2023/blob/master/src/bin/2.rs\

pub fn part_one(input: &str) -> Option<u32> {
    // You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like
    // the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5
    // green, 4 blue).

    // For example, the record of a few games might look like this:

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

    // In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes;
    // the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

    // The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and
    // 14 blue cubes?
    let input_as_string = input.to_string();
    let mut result: u32 = 0;
    for line in input_as_string.lines() {
        let (id, data) = line.split_once(": ").unwrap();
        let game: Vec<&str> = data.split("; ").into_iter().collect();
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;
        for round in game {
            for cube in round.split(", ") {
                let amount = cube
                    .to_string()
                    .split(|c: char| !c.is_numeric() && c != '-')
                    .filter_map(|x| {
                        if x.is_empty() {
                            None
                        } else {
                            Some(x.trim().to_string().trim().parse().unwrap())
                        }
                    })
                    .next()
                    .unwrap();
                let color = cube.split(' ').nth(1).unwrap();
                if color == "blue" {
                    blue = blue.max(amount);
                }
                if color == "red" {
                    red = red.max(amount);
                }
                if color == "green" {
                    green = green.max(amount);
                }
            }
        }
        if red <= 12 && green <= 13 && blue <= 14 {
            result += id
                .to_string()
                .split(|c: char| !c.is_numeric() && c != '-')
                .filter_map(|x| {
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.trim().parse::<u32>().unwrap())
                    }
                })
                .next()
                .unwrap();
        }
    }
    return Some(result);
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_as_string = input.to_string();
    let mut result: u32 = 0;
    for line in input_as_string.lines() {
        let (id, data) = line.split_once(": ").unwrap();
        let game: Vec<&str> = data.split("; ").into_iter().collect();
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;
        for round in game {
            for cube in round.split(", ") {
                let amount = cube
                    .to_string()
                    .split(|c: char| !c.is_numeric() && c != '-')
                    .filter_map(|x| {
                        if x.is_empty() {
                            None
                        } else {
                            Some(x.trim().to_string().trim().parse().unwrap())
                        }
                    })
                    .next()
                    .unwrap();
                let color = cube.split(' ').nth(1).unwrap();
                if color == "blue" {
                    blue = blue.max(amount);
                }
                if color == "red" {
                    red = red.max(amount);
                }
                if color == "green" {
                    green = green.max(amount);
                }
            }
        }
        result += green * red * blue;
    }
    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

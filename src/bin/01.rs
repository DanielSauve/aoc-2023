advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut nums: Vec<u32> = vec![];
    for line in input.lines() {
        let mut numbers: Vec<char> = vec![];
        for c in line.chars() {
            if c.is_numeric() {
                numbers.push(c);
            }
        }
        let mut number: u32 = numbers.first().unwrap().to_digit(10).unwrap();
        number *= 10;
        number += numbers.last().unwrap().to_digit(10).unwrap();

        nums.push(number);
    }
    Some(nums.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut nums: Vec<u32> = vec![];
    let words: Vec<&str> = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    for line in input.lines() {
        let mut min_word: &str = "";
        let mut max_word: &str = "";
        let mut min_index = usize::MAX;
        let mut max_index = usize::MIN;
        for word in &words {
            let index = line.find(word);
            match index {
                None => continue,
                Some(x) => {
                    if x < min_index {
                        min_index = x;
                        min_word = word;
                    }
                    if x > max_index || max_index == 0 {
                        max_index = x;
                        max_word = word;
                    }
                }
            }
            let other_index = line.rfind(word);
            match other_index {
                None => continue,
                Some(x) => {
                    if x < min_index {
                        min_index = x;
                        min_word = word;
                    }
                    if x > max_index || max_index == 0 {
                        max_index = x;
                        max_word = word;
                    }
                }
            }
        }
        if min_word.is_empty() && max_word.is_empty() {
            continue;
        }

        let mut number: u32 = match min_word {
            "one" | "1" => 10,
            "two" | "2" => 20,
            "three" | "3" => 30,
            "four" | "4" => 40,
            "five" | "5" => 50,
            "six" | "6" => 60,
            "seven" | "7" => 70,
            "eight" | "8" => 80,
            "nine" | "9" => 90,
            _ => panic!("This is not possible"),
        };

        number += match max_word {
            "one" | "1" => 1,
            "two" | "2" => 2,
            "three" | "3" => 3,
            "four" | "4" => 4,
            "five" | "5" => 5,
            "six" | "6" => 6,
            "seven" | "7" => 7,
            "eight" | "8" => 8,
            "nine" | "9" => 9,
            _ => panic!("This is not possible"),
        };
        nums.push(number);
    }
    Some(nums.iter().sum())
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}

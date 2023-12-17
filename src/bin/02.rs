advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut possible_total = 0;
    for game in input.lines() {
        let mut possible = true;
        let (game_id, sets_str) = game.split_once(':')?;
        let game_result = game_id.split_once(' ')?.1.parse();
        let game_num: u32 = match game_result {
            Ok(x) => x,
            _ => 0,
        };
        let sets = sets_str.split(';');
        for set in sets {
            let marbles = set.split(',');
            for marble in marbles {
                if !possible {
                    break;
                }
                let strip_marble = marble.strip_prefix(' ')?;
                let (count_str, colour) = strip_marble.split_once(' ')?;
                let count: u32 = match count_str.parse() {
                    Ok(x) => x,
                    _ => 0,
                };
                possible = match colour {
                    "red" => count <= max_red,
                    "blue" => count <= max_blue,
                    "green" => count <= max_green,
                    _ => false,
                };
            }
        }
        if possible {
            possible_total += game_num;
        }
    }
    Some(possible_total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut possible_total = 0;
    for game in input.lines() {
        let (_, sets_str) = game.split_once(':')?;
        let sets = sets_str.split(';');
        let mut red_min = 0;
        let mut blue_min = 0;
        let mut green_min = 0;
        for set in sets {
            let marbles = set.split(',');
            for marble in marbles {
                let strip_marble = marble.strip_prefix(' ')?;
                let (count_str, colour) = strip_marble.split_once(' ')?;
                let count: u32 = match count_str.parse() {
                    Ok(x) => x,
                    _ => 0,
                };
                match colour {
                    "red" => red_min = red_min.max(count),
                    "green" => green_min = green_min.max(count),
                    "blue" => blue_min = blue_min.max(count),
                    _ => (),
                }
            }
        }
        possible_total += red_min * green_min * blue_min;
    }
    Some(possible_total)
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
        assert_eq!(result, Some(2286));
    }
}

use std::{collections::HashMap};

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<u32, AocError> {
    let input = augment_input(_input);
    let star_positions = get_stars_positions(&input);
    let numbers = get_numbers_adjustent_to_the_stars(&star_positions, &input);
    let res = numbers
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .map(|(_, v)| v.iter().product::<u32>())
        .sum();

    Ok(res)
}

fn augment_input(input: &str) -> String {
    let mut output = String::new();
    let first_line = String::from(".").repeat(input.lines().next().unwrap().len() + 2);
    output.push_str(first_line.as_str());
    output.push_str("\n");
    for line in input.lines() {
        output.push_str(String::from(".").as_str());
        output.push_str(line.trim());
        output.push_str(String::from(".").as_str());
        output.push_str("\n");
    }
    output.push_str(first_line.as_str());
    output.push_str("\n");
    output
}

fn get_stars_positions(input: &str) -> Vec<(usize, usize)> {
    let mut stars_positions: Vec<(usize, usize)> = Vec::new();
    let lines_vec: Vec<&str> = input.lines().collect();

    for (i, line) in lines_vec.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '*' {
                stars_positions.push((i, j));
            }
        }
    }
    stars_positions
}

fn parse_number(line: &str, pos: usize) -> Option<u32> {
    let mut s = String::new();

    if !line.chars().nth(pos).unwrap().is_digit(10) {
        return None;
    }

    let mut left = pos;
    let mut right = pos + 1;
    while line.chars().nth(left).unwrap().is_digit(10) {
        s.insert_str(0, &line.chars().nth(left).unwrap().to_string());
        if left == 0 {
            break;
        }
        left -= 1;
    }

    while line.chars().nth(right).unwrap().is_digit(10) {
        s.push(char::from(line.chars().nth(right).unwrap()));
        if right == line.len() - 1 {
            break;
        }
        right += 1;
    }

    match s.parse() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

fn get_number_at_location(input: &str, location: (usize, usize)) -> Option<u32> {
    let lines_vec: Vec<&str> = input.lines().collect();
    let (i, j) = location;
    parse_number(lines_vec[i], j)
}

fn get_numbers_adjustent_to_the_stars(
    star_positions: &Vec<(usize, usize)>,
    input: &str,
) -> HashMap<(u32, u32), Vec<u32>> {
    let mut numbers: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    for (i, j) in star_positions.iter() {
        let top_left = get_number_at_location(input, (*i - 1, *j - 1));
        let top = get_number_at_location(input, (*i - 1, *j));
        let top_right = get_number_at_location(input, (*i - 1, *j + 1));
        let left = get_number_at_location(input, (*i, *j - 1));
        let right = get_number_at_location(input, (*i, *j + 1));
        let bottom_left = get_number_at_location(input, (*i + 1, *j - 1));
        let bottom = get_number_at_location(input, (*i + 1, *j));
        let bottom_right = get_number_at_location(input, (*i + 1, *j + 1));
        let numbers_vec = vec![
            top_left,
            top,
            top_right,
            left,
            right,
            bottom_left,
            bottom,
            bottom_right,
        ];
        let unique_numbers_vec = numbers_vec
            .iter()
            .flatten()
            .unique()
            .cloned()
            .collect::<Vec<u32>>();
        numbers.insert((*i as u32, *j as u32), unique_numbers_vec);
    }
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        assert_eq!(467835, process(input)?);
        Ok(())
    }

    #[test]
    fn test_augment_input() {
        let input = "123
        456
        789";
        let expected = ".....\n.123.\n.456.\n.789.\n.....\n";
        assert_eq!(expected, augment_input(input));
    }

    #[test]
    fn test_get_stars_positions() {
        let input = "467..114..
                           ...*......
                           ..35..633.
                           ......#...
                           617*......
                           .....+.58.
                           ..592.....
                           ......755.
                           ...$.*....
                           .664.598..";
        let i = augment_input(input);
        let expected = vec![(2, 4), (5, 4), (9, 6)];
        assert_eq!(expected, get_stars_positions(&i));
    }

    #[test]
    fn test_parse_number() {
        let i = "467..114..";
        assert_eq!(467, parse_number(&i, 0).unwrap());
        assert_eq!(467, parse_number(&i, 1).unwrap());
        assert_eq!(467, parse_number(&i, 2).unwrap());
        assert_eq!(0, parse_number(&i, 3).unwrap_or(0));
        assert_eq!(0, parse_number(&i, 4).unwrap_or(0));

        assert_eq!(114, parse_number(&i, 5).unwrap());
        assert_eq!(114, parse_number(&i, 6).unwrap());
        assert_eq!(114, parse_number(&i, 7).unwrap());

        assert_eq!(0, parse_number(&i, 8).unwrap_or(0));
        assert_eq!(0, parse_number(&i, 9).unwrap_or(0));
    }

    #[test]
    fn test_get_number_at_location() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        let i = augment_input(input);
        assert_eq!(Some(467), get_number_at_location(&i, (1, 1)));
        assert_eq!(Some(467), get_number_at_location(&i, (1, 2)));
        assert_eq!(Some(467), get_number_at_location(&i, (1, 3)));

        assert_eq!(None, get_number_at_location(&i, (1, 4)));
        assert_eq!(None, get_number_at_location(&i, (1, 5)));

        assert_eq!(Some(114), get_number_at_location(&i, (1, 6)));
        assert_eq!(Some(114), get_number_at_location(&i, (1, 7)));
        assert_eq!(Some(114), get_number_at_location(&i, (1, 8)));

        assert_eq!(Some(35), get_number_at_location(&i, (3, 3)));
        assert_eq!(Some(35), get_number_at_location(&i, (3, 4)));

        assert_eq!(None, get_number_at_location(&i, (2, 4)));
    }

    #[test]
    fn test_result() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        let i = augment_input(input);
        assert_eq!(467835, process(&i).unwrap_or(0));

    }
}

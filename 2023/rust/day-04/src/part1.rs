use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    multi::separated_list1,
    sequence::{pair, preceded, delimited},
    IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<u32, AocError> {
    let games = _input
        .lines()
        .map(|line| parse_card(line).unwrap().1)
        .collect::<Vec<_>>();

    print!("{:?}", games);
    let sum = games
        .iter()
        .map(|card| {
            let points = card.get_points();
            tracing::info!("Card {} has {} points", card.id, points);
            points
        })
        .sum::<u32>();
    Ok(sum)
}

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    victory_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn get_points(&self) -> u32 {
        let matches = self
            .numbers
            .iter()
            .filter(|n| self.victory_numbers.contains(n));
        let count = matches.count();
        println!("Card {} has {} matches", self.id, count);
        match count {
            0 => 0,
            1 => 1,
            _ => (2 as u32).pow(count as u32 - 1),
        }
    }
}

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = delimited(space1, tag("|"), space1)(input)?;
    let (input, numbers) = separated_list1(space1, digit1)(input)?;
    let numbers = numbers.into_iter().map(|n| n.parse().unwrap()).collect();
    Ok((input, numbers))
}

fn victory_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = pair(tag(":"), space1)(input)?;
    let (input, victory_numbers) = separated_list1(space1, digit1)(input)?;
    let victory_numbers = victory_numbers
        .into_iter()
        .map(|n| n.parse().unwrap())
        .collect();
    Ok((input, victory_numbers))
}

fn card_id(input: &str) -> IResult<&str, u32> {
    let (input, id) = preceded(pair(pair(space0, tag("Card")), space1), digit1)(input)?;
    Ok((input, id.parse().unwrap()))
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, id) = card_id(input)?;
    let (input, victory_numbers) = victory_numbers(input)?;
    let (input, numbers) = numbers(input)?;
    let victory_numbers = victory_numbers.into_iter().collect();
    Ok((
        input,
        Card {
            id,
            victory_numbers,
            numbers,
        },
    ))
}

fn parse_card(line: &str) -> IResult<&str, Card> {
    let (line, card) = card(line)?;
    Ok((line, card))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card    1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, process(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_card() {
        let input = "           Card    1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (_, card) = parse_card(input).unwrap();
        let expected_hash = vec![41, 48, 83, 86, 17].into_iter().collect();
        let expected_numbers = vec![83, 86, 6, 31, 17, 9, 48, 53];
        assert_eq!(card.id, 1);
        assert_eq!(card.victory_numbers, expected_hash);
        assert_eq!(card.numbers, expected_numbers);
        assert_eq!(card.get_points(), 8);
    }
}

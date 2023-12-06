use std::{collections::HashMap, ops::Range};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, space0, space1, u64},
    multi::{fold_many1, separated_list1},
    sequence::{pair, preceded},
    IResult,
};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (seeds, map) = parse_input(_input).unwrap().1;
    println!("{:?}", seeds);
    let result = walk_the_map(&map, &seeds);
    let min = result.iter().min().unwrap();
    Ok((*min).to_string())
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(space1, digit1)(input)?;
    let seeds = seeds.into_iter().map(|n| n.parse().unwrap()).collect();
    let (input, _) = preceded(space0, tag("\n"))(input)?;
    Ok((input, seeds))
}

fn throw_empty_line(input: &str) -> IResult<&str, &str> {
    let (input, _) = pair(space0, tag("\n"))(input)?;
    Ok((input, input))
}

fn parse_line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, _) = space0(input)?;
    let (input, dest) = u64(input)?;
    let (input, _) = space1(input)?;
    let (input, source) = u64(input)?;
    let (input, _) = space1(input)?;
    let (input, num) = u64(input)?;
    let (input, _) = preceded(space0,tag("\n"))(input)?;
    Ok((input, (source..source + num, dest..dest + num)))
}

fn parse_block(input: &str) -> IResult<&str, Vec<(Range<u64>, Range<u64>)>> {
    let (input, _header) = preceded(pair(space0, take_until("\n")), tag("\n"))(input)?;
    let (input, map) = fold_many1(parse_line, Vec::new, |mut acc, item| {
        acc.push(item);
        acc
    })(input)?;
    Ok((input, map))
}

fn parse_map(input: &str) -> IResult<&str, Vec<Vec<(Range<u64>, Range<u64>)>>> {
    let (input, seed_to_soil) = parse_block(input)?;
    let (input, _) = throw_empty_line(input)?;
    let (input, soil_to_fertilizer) = parse_block(input)?;
    let (input, _) = throw_empty_line(input)?;
    let (input, fertilizer_to_water) = parse_block(input)?;
    let (input, _) = throw_empty_line(input)?;
    let (input, water_to_light) = parse_block(input)?;
    let (input, _) = throw_empty_line(input)?;
    let (input, light_to_temperature) = parse_block(input)?;
    let (input, _) = throw_empty_line(input)?;
    let (input, temperature_to_humidity) = parse_block(input)?;
    let (input, _) = throw_empty_line(input)?;
    let (input, humidity_to_location) = parse_block(input)?;
    let map = vec![
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ];
    Ok((input, map))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<Vec<(Range<u64>, Range<u64>)>>)> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = throw_empty_line(input)?;
    let (input, map) = parse_map(input)?;
    Ok((input, (seeds, map)))
}

fn walk_the_map(
    maps: &Vec<Vec<(Range<u64>, Range<u64>)>>,
    seeds: &Vec<u64>,
) -> Vec<u64> {
    seeds
        .iter()
        .map(|seed| {
            let mut seed = *seed;
            for map in maps {
                for (source, dest) in map {
                    if source.contains(&seed) {
                        seed = dest.start + (seed - source.start);
                        break;
                    }
                }
            }
            seed
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";
        assert_eq!("35", process(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 14 55 13
        
        
        seed-to-soil map:";
        let (_, seeds) = parse_seeds(input).unwrap();
        assert_eq!(vec![79, 14, 55, 13], seeds);
    }

    #[test]
    fn test_parse_seeds2() {
        let input = "seeds: 304740406 53203352 1080760686 52608146 1670978447 367043978 1445830299 58442414 4012995194 104364808 4123691336 167638723 2284615844 178205532 3164519436 564398605 90744016 147784453 577905361 122056749
        
        
        seed-to-soil map:";
        let (_, seeds) = parse_seeds(input).unwrap();
        assert_eq!(vec![304740406,53203352,1080760686,52608146,1670978447,367043978,1445830299,58442414,4012995194,104364808,4123691336,167638723,2284615844,178205532,3164519436,564398605,90744016,147784453,577905361,122056749], seeds);
    }

    #[test]
    fn test_throw_line() {
        let input = "seeds: 79 14 55 13
        
        
        seed-to-soil map:";
        let (input, _) = parse_seeds(input).unwrap();
        let (input, _) = throw_empty_line(input).unwrap();
        let (input, _) = throw_empty_line(input).unwrap();
        assert_eq!("seed-to-soil map:", input.trim());
    }

    #[test]
    fn parse_input_test() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        let (_, (seeds, map)) = parse_input(input).unwrap();
        assert_eq!(vec![79, 14, 55, 13], seeds);
        assert_eq!(map.len(), 7);
    }
    #[test]
    fn map_walk() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        let (_, (seeds, map)) = parse_input(input).unwrap();
        let result = walk_the_map(&map, &seeds);
        assert_eq!(result, vec![82, 43, 86, 35]);
    }
}

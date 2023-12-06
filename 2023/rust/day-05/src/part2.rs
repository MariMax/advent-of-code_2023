use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{space0, space1, u64},
    multi::{fold_many1, separated_list1},
    sequence::{pair, preceded},
    IResult,
};
use rayon::{iter::IntoParallelIterator, iter::ParallelIterator};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (seeds, map) = parse_input(_input).unwrap().1;
    let result = walk_the_map(&map, seeds);
    let min = result.iter().min().unwrap();
    Ok((*min).to_string())
}

fn parse_seeds_pair(input: &str) -> IResult<&str, Range<u64>> {
    let (input, _) = space0(input)?;
    let (input, start) = u64(input)?;
    let (input, _) = space1(input)?;
    let (input, end) = u64(input)?;
    Ok((input, start..start + end))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(space1, parse_seeds_pair)(input)?;
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
    let (input, _) = preceded(space0, tag("\n"))(input)?;
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

fn parse_input(
    input: &str,
) -> IResult<&str, (Vec<Range<u64>>, Vec<Vec<(Range<u64>, Range<u64>)>>)> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = throw_empty_line(input)?;
    let (input, map) = parse_map(input)?;
    Ok((input, (seeds, map)))
}

fn walk_the_map(maps: &Vec<Vec<(Range<u64>, Range<u64>)>>, seeds_vec: Vec<Range<u64>>) -> Vec<u64> {
    // let mut min = u64::MAX;
    let all_seeds = seeds_vec.iter().map(|s| s.clone().count() as u64).sum::<u64>();
    let bar = indicatif::ProgressBar::new(all_seeds);
    let min = seeds_vec.into_par_iter()
    .map(|seeds| {
        seeds.clone().into_par_iter()
        .map(|seed| {
            bar.inc(1);
            let mut result = seed;
            for map in maps {
                for (source, dest) in map {
                    if source.contains(&result) {
                        result = dest.start + (result - source.start);
                        break;
                    }
                }
            }
            result
        })
    })
    .flatten()
    .collect();
    min
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
        assert_eq!("46", process(input)?);
        Ok(())
    }
}

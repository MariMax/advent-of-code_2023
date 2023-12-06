use std::collections::HashMap;

#[tracing::instrument]
pub fn process(_input: &str) -> Result<i32, &str> {
    let mut total = 0;
    for line in _input.lines() {
        let id = get_game_id(line)?;
        let sets = get_game_sets(line)?;
        let mut is_possible = true;
        for set in sets.iter() {
            let map = parse_set(set)?;
            if !is_game_possible(&map) {
                is_possible = false;
                break;
            }
        }
        if is_possible {
            total += id;
        }
    }
    Ok(total)
}

fn get_game_id(input: &str) -> Result<i32, &str> {
    let parts: Vec<&str> = input.split_terminator(":").collect();
    if parts.len() < 2 {
        return Err("Invalid input");
    }
    let id_parts: Vec<&str> = parts[0].split_whitespace().collect();
    if id_parts.len() < 2 {
        return Err("Invalid input");
    }
    let id = id_parts[1];
    let num_id = id.parse::<i32>().unwrap_or(0);
    Ok(num_id)
}

fn get_game_sets(input: &str) -> Result<Vec<&str>, &str> {
    let parts: Vec<&str> = input.split_terminator(":").collect();
    if parts.len() < 2 {
        return Err("Invalid input");
    }
    let sets = parts[1].split_terminator(";").map(|s| s.trim()).collect();
    Ok(sets)
}

// fn parse_sets(sets: Vec<&str>) -> Result<HashMap<&str, i32>, &str> {
//     let mut map: HashMap<&str, i32> = HashMap::new();
//     for set in sets.iter() {
//         let games: Vec<&str> = set.split_terminator(",").map(|s| s.trim()).collect();
//         for game in games.iter()  {
//             let play: Vec<&str> = game.split_whitespace().collect();
//             let color = play[1];
//             let num = play[0].parse::<i32>().unwrap_or(0);
//             let val: i32 = if map.contains_key(&color) {
//                 *map.get(&color).unwrap() + num
//             } else {
//                 num
//             };
//             map.insert(color, val);
//         }
//     }
//     Ok(map)
// }

fn parse_set(set: &str) -> Result<HashMap<&str, i32>, &str> {
    let mut map: HashMap<&str, i32> = HashMap::new();
    let games: Vec<&str> = set.split_terminator(",").map(|s| s.trim()).collect();
    for game in games.iter()  {
        let play: Vec<&str> = game.split_whitespace().collect();
        let color = play[1];
        let num = play[0].parse::<i32>().unwrap_or(0);
        let val: i32 = if map.contains_key(&color) {
            *map.get(&color).unwrap() + num
        } else {
            num
        };
        map.insert(color, val);
    }
    Ok(map)
}

fn is_game_possible(map: &HashMap<&str, i32>) -> bool {
    //12 red, 13 green, 14 blue
    let red = map.get("red").unwrap_or(&0);
    let green = map.get("green").unwrap_or(&0);
    let blue = map.get("blue").unwrap_or(&0);
    red <= &12 && green <= &13 && blue <= &14
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, process(input).unwrap_or(0));
        Ok(())
    }

    #[test]
    fn test_get_game_id() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(1, get_game_id(input).unwrap());
    }

    #[test]
    fn test_get_game_sets() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = vec!["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"];
        assert_eq!(expected, get_game_sets(input).unwrap());
    }

    // #[test]
    // fn test_parse_set() {
    //     let input = "Game 1: 3 blue, 4 red; 1 red, 2 green";
    //     let mut expected: HashMap<&str, i32> = HashMap::new();
    //     expected.insert("blue", 3);
    //     expected.insert("red", 5);
    //     expected.insert("green", 2);
    //     let sets = get_game_sets(input).unwrap();
    //     assert_eq!(expected, parse_sets(sets).unwrap());
    // }
}

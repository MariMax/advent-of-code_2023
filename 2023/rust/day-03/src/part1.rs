use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<u32, AocError> {
    let input = augment_input(_input);
    let valid_numbers = get_valid_numbers(&input);
    Ok(valid_numbers.iter().sum())
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

fn get_valid_numbers(input: &str) -> Vec<u32> {
    let mut valid_numbers: Vec<u32> = Vec::new();
    let lines_vec: Vec<&str> = input.lines().collect();

    let mut s = String::new();
    let mut is_valid = false;

    for (i, line) in lines_vec.iter().enumerate() {        

        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                s.push(c);
                if (!(lines_vec[i - 1].chars().nth(j - 1).unwrap()).is_digit(10)
                    && (lines_vec[i - 1].chars().nth(j - 1).unwrap()) != '.')
                    || (!(lines_vec[i - 1].chars().nth(j).unwrap()).is_digit(10)
                        && (lines_vec[i - 1].chars().nth(j).unwrap()) != '.')
                    || (!(lines_vec[i - 1].chars().nth(j + 1).unwrap()).is_digit(10)
                        && (lines_vec[i - 1].chars().nth(j + 1).unwrap()) != '.')
                    || (!(lines_vec[i + 1].chars().nth(j - 1).unwrap()).is_digit(10)
                        && (lines_vec[i + 1].chars().nth(j - 1).unwrap()) != '.')
                    || (!(lines_vec[i + 1].chars().nth(j).unwrap()).is_digit(10)
                        && (lines_vec[i + 1].chars().nth(j).unwrap()) != '.')
                    || (!(lines_vec[i + 1].chars().nth(j + 1).unwrap()).is_digit(10)
                        && (lines_vec[i + 1].chars().nth(j + 1).unwrap()) != '.')
                    || (!(lines_vec[i].chars().nth(j - 1).unwrap()).is_digit(10)
                        && (lines_vec[i].chars().nth(j - 1).unwrap()) != '.')
                    || (!(lines_vec[i].chars().nth(j + 1).unwrap()).is_digit(10)
                        && (lines_vec[i].chars().nth(j + 1).unwrap()) != '.')
                {
                    is_valid = true;
                }
            } else {
                let num = if s.is_empty() {
                    0
                } else {
                    s.parse::<u32>().unwrap()
                };
                if is_valid {
                    valid_numbers.push(num);
                }
                s.clear();
                is_valid = false;
            }
        }
    }
    valid_numbers
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
        assert_eq!(4361, process(input)?);
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
    fn test_get_valid_numbers() {
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
        let expected = vec![467, 35, 633, 617, 592, 755, 664, 598];
        assert_eq!(expected, get_valid_numbers(&i));
    }
}

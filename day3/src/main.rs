use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, PartialEq)]
struct PartNumber {
    num: u32,
    start: usize,
    end: usize,
}

fn main() {
    let lines = read_lines("input.txt");
}

/// Read all lines into a Vec<String> crash if something isn't right
fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);

    buf.lines().map(|l| l.unwrap()).collect()
}

fn find_numbers(line: &String) -> Vec<PartNumber> {
    let mut parts = Vec::new();

    let mut num_chars = String::from("");
    let mut start: usize = 0;
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            if i != 0 && !line.chars().collect::<Vec<char>>()[i - 1].is_numeric() {
                start = i;
            }
            num_chars.push(c);
            if i + 1 > line.chars().collect::<Vec<_>>().len() {
                let part = PartNumber {
                    num: num_chars.parse::<u32>().unwrap(),
                    end: i,
                    start,
                };
                parts.push(part);
                num_chars = String::from("");
            } else {
                if !line.chars().collect::<Vec<char>>()[i + 1].is_numeric() {
                    let part = PartNumber {
                        num: num_chars.parse::<u32>().unwrap(),
                        end: i,
                        start,
                    };
                    parts.push(part);
                    num_chars = String::from("");
                }
            }
        }
    }

    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::find_numbers;

    #[test]
    fn pt1_functional_test() {
        let test_lines = vec![
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598.."),
        ];
        let expected = 4361;
    }

    #[test]
    fn test_find_numbers() {
        let test = String::from("467..114..");

        let found_parts = find_numbers(&test);

        let expected_parts = vec![
            PartNumber {
                num: 467,
                start: 0,
                end: 2,
            },
            PartNumber {
                num: 114,
                start: 5,
                end: 7,
            },
        ];
        assert_eq!(found_parts, expected_parts)
    }
}

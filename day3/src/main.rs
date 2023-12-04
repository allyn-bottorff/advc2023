use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, PartialEq)]
struct PartNumber {
    num: u32,
    start: usize,
    end: usize,
    found: bool,
}

fn main() {
    let lines = read_lines("input.txt");

    let mut parts: Vec<Vec<PartNumber>> = lines
        .clone()
        .into_iter()
        .map(|l| find_numbers(&l))
        .collect();

    let mut sum = 0;
    let mut power_sum = 0;

    for (i, line) in lines.into_iter().enumerate() {
        let (part_val, power) = find_adjacent_parts(&line, i, &mut parts);
        sum += part_val;
        power_sum += power;
    }

    println!("Pt 1 sum: {}", sum);
    println!("Pt. 2 sum: {}", power_sum);
}

/// Traverse a line and find symbols. Then check if the adjacent cells are
/// contained within a part.
fn find_adjacent_parts(
    line: &String,
    l_num: usize,
    parts: &mut Vec<Vec<PartNumber>>,
) -> (u32, u32) {
    let mut part_sum = 0;
    let mut power_sum = 0;
    for (i, c) in line.chars().enumerate() {
        if !c.is_ascii_digit() && c != '.' {
            let mut adj = Vec::new();
            // Check current line
            for part in &mut parts[l_num] {
                if i > 0 {
                    if part.start <= i - 1 && part.end >= i - 1 {
                        if !part.found {
                            if c == '*' {
                                adj.push(part.num.clone());
                            }
                            part_sum += part.num;
                            part.found = true;
                        }
                    }
                }
                if part.start <= i + 1 && part.end >= i + 1 {
                    if !part.found {
                        if c == '*' {
                            adj.push(part.num.clone());
                        }
                        part_sum += part.num;
                        part.found = true;
                    }
                }
            }
            // Check previous line
            if l_num > 0 {
                for part in &mut parts[l_num - 1] {
                    if i > 0 {
                        if part.start <= i - 1 && part.end >= i - 1 {
                            if !part.found {
                                if c == '*' {
                                    adj.push(part.num.clone());
                                }
                                part_sum += part.num;
                                part.found = true;
                            }
                        }
                    }
                    if part.start <= i && part.end >= i {
                        if !part.found {
                            if c == '*' {
                                adj.push(part.num.clone());
                            }
                            part_sum += part.num;
                            part.found = true;
                        }
                    }
                    if part.start <= i + 1 && part.end >= i + 1 {
                        if !part.found {
                            if c == '*' {
                                adj.push(part.num.clone());
                            }
                            part_sum += part.num;
                            part.found = true;
                        }
                    }
                }
            }
            // Check next line
            if l_num < parts.len().clone() - 1 {
                for part in &mut parts[l_num + 1] {
                    if i > 0 {
                        if part.start <= i - 1 && part.end >= i - 1 {
                            if !part.found {
                                if c == '*' {
                                    adj.push(part.num.clone());
                                }
                                part_sum += part.num;
                                part.found = true;
                            }
                        }
                    }
                    if part.start <= i && part.end >= i {
                        if !part.found {
                            if c == '*' {
                                adj.push(part.num.clone());
                            }
                            part_sum += part.num;
                            part.found = true;
                        }
                    }
                    if part.start <= i + 1 && part.end >= i + 1 {
                        if !part.found {
                            if c == '*' {
                                adj.push(part.num.clone());
                            }
                            part_sum += part.num;
                            part.found = true;
                        }
                    }
                }
            }
            if adj.len() == 2 {
                let power = adj[0] * adj[1];
                power_sum += power;
            }
        }
    }

    (part_sum, power_sum)
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
        if c.is_ascii_digit() {
            if i != 0 && !line.chars().collect::<Vec<char>>()[i - 1].is_ascii_digit() {
                start = i;
            }
            num_chars.push(c);
            if i + 1 >= line.chars().collect::<Vec<_>>().len() {
                let part = PartNumber {
                    num: num_chars.parse::<u32>().unwrap(),
                    end: i,
                    found: false,
                    start,
                };
                parts.push(part);
                num_chars = String::from("");
            } else {
                if !line.chars().collect::<Vec<char>>()[i + 1].is_ascii_digit() {
                    let part = PartNumber {
                        num: num_chars.parse::<u32>().unwrap(),
                        end: i,
                        found: false,
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

        let mut sum = 0;

        let mut parts: Vec<Vec<PartNumber>> = test_lines
            .clone()
            .into_iter()
            .map(|l| find_numbers(&l))
            .collect();

        for (i, line) in test_lines.into_iter().enumerate() {
            let (part_sum, _) = find_adjacent_parts(&line, i, &mut parts);
            sum += part_sum;
        }

        let expected = 4361;
        assert_eq!(sum, expected)
    }

    #[test]
    fn pt1_find_adjacent_parts_line_0() {
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
        let mut parts: Vec<Vec<PartNumber>> = test_lines
            .clone()
            .into_iter()
            .map(|l| find_numbers(&l))
            .collect();
        let (sum, _) = find_adjacent_parts(&test_lines[0], 0, &mut parts);
        assert_eq!(0, sum)
    }

    #[test]
    fn pt1_find_adjacent_parts_line_1() {
        let test_lines = vec![
            String::from("467..114.."),
            String::from("...*.....$"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598.."),
        ];
        let mut parts: Vec<Vec<PartNumber>> = test_lines
            .clone()
            .into_iter()
            .map(|l| find_numbers(&l))
            .collect();
        let (sum, _) = find_adjacent_parts(&test_lines[1], 1, &mut parts);
        assert_eq!(467 + 35 + 633, sum)
    }
    #[test]
    fn pt1_find_adjacent_parts_line_3() {
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
        let mut parts: Vec<Vec<PartNumber>> = test_lines
            .clone()
            .into_iter()
            .map(|l| find_numbers(&l))
            .collect();
        let (sum, _) = find_adjacent_parts(&test_lines[3], 3, &mut parts);
        assert_eq!(633, sum)
    }

    #[test]
    fn pt1_find_adjacent_parts_line_8() {
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
        let mut parts: Vec<Vec<PartNumber>> = test_lines
            .clone()
            .into_iter()
            .map(|l| find_numbers(&l))
            .collect();
        let (sum, _) = find_adjacent_parts(&test_lines[8], 8, &mut parts);
        assert_eq!(664 + 598 + 755, sum)
    }

    #[test]
    fn pt2_find_gear_power_1() {
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
        let mut parts: Vec<Vec<PartNumber>> = test_lines
            .clone()
            .into_iter()
            .map(|l| find_numbers(&l))
            .collect();
        let (_, power) = find_adjacent_parts(&test_lines[1], 1, &mut parts);
        assert_eq!(467 * 35, power)
    }

    #[test]
    fn pt2_find_gear_power_2() {
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
        let mut parts: Vec<Vec<PartNumber>> = test_lines
            .clone()
            .into_iter()
            .map(|l| find_numbers(&l))
            .collect();
        let (_, power) = find_adjacent_parts(&test_lines[2], 2, &mut parts);
        assert_eq!(0, power)
    }
    #[test]
    fn pt2_find_gear_power_8() {
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
        let mut parts: Vec<Vec<PartNumber>> = test_lines
            .clone()
            .into_iter()
            .map(|l| find_numbers(&l))
            .collect();
        let (_, power) = find_adjacent_parts(&test_lines[8], 8, &mut parts);
        assert_eq!(755 * 598, power)
    }

    #[test]
    fn test_find_numbers_1() {
        let test = String::from("467..114..");

        let found_parts = find_numbers(&test);

        let expected_parts = vec![
            PartNumber {
                num: 467,
                start: 0,
                end: 2,
                found: false,
            },
            PartNumber {
                num: 114,
                start: 5,
                end: 7,
                found: false,
            },
        ];
        assert_eq!(found_parts, expected_parts)
    }

    #[test]
    fn test_find_numbers_2() {
        let test = String::from("..35..633.");

        let found_parts = find_numbers(&test);

        let expected_parts = vec![
            PartNumber {
                num: 35,
                start: 2,
                end: 3,
                found: false,
            },
            PartNumber {
                num: 633,
                start: 6,
                end: 8,
                found: false,
            },
        ];
        assert_eq!(found_parts, expected_parts)
    }
    #[test]
    fn test_find_numbers_3() {
        let test = String::from("..35..633");

        let found_parts = find_numbers(&test);

        let expected_parts = vec![
            PartNumber {
                num: 35,
                start: 2,
                end: 3,
                found: false,
            },
            PartNumber {
                num: 633,
                start: 6,
                end: 8,
                found: false,
            },
        ];
        assert_eq!(found_parts, expected_parts)
    }
}

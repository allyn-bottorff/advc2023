use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("input.txt");

    let mut sum = 0;
    for line in lines {
        sum += parse_line_digits(line);

    }
    println!("Sum: {}", sum)
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let buf = io::BufReader::new(file);
    buf.lines().map(|l| l.unwrap()).collect()
}

fn parse_line_digits(line: String) -> u32 {

    let num_chars: Vec<char> = line.chars()
        .filter(|c| c.is_numeric()).collect();


    let first = num_chars.first().unwrap_or_else(|| &'0');

    let last = num_chars.last().unwrap_or_else(|| &'0');

    let mut num_string = String::from("");
    num_string.push(*first);
    num_string.push(*last);

    let num = num_string.parse::<u32>().unwrap();
    num

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_tests() {
        assert_eq!(1, 1)
    }

    #[test]
    fn test_parse_line() {
        let line: String = "1abc2".into();
        let parsed = parse_line_digits(line);
        assert_eq!(parsed, 12)
    }

    #[test]
    fn test_calibration() {
        let lines: Vec<String> = vec![
            "1abc2".into(),
            "pqr3stu8vwx".into(),
            "a1b2c3d4e5f".into(),
            "treb7uchet".into(),
        ];

        let mut sum = 0;

        for line in lines {
            sum += parse_line_digits(line);
        }

        assert_eq!(sum, 142)



    }
}

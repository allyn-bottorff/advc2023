use itertools::Itertools;
use std::ops::Range;

#[derive(Debug)]
struct Record {
    rules: Vec<u8>,
    springs: String,
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let (spring_str, rules_str) = value.split_once(" ").unwrap();
        let rules: Vec<u8> = rules_str
            .split(",")
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        Record {
            rules,
            springs: spring_str.to_string(),
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("example.txt").unwrap();
    let records: Vec<Record> = contents.lines().map(|l| Record::from(l)).collect();

    println!("{:?}", records);

    let test_string = String::from("????");
    let mut chars = vec!['.'; test_string.len()];
    let mut pounds = vec!['#'; test_string.len()];
    chars.append(&mut pounds);
    let result: Vec<String> = chars
        .into_iter()
        .permutations(test_string.len())
        .map(|x| x.into_iter().collect::<String>())
        .unique()
        .collect();
    println!();

    for line in &records{
        println!("{:?}", line.springs);
    }

    let mut qs: Vec<Vec<Range<usize>>> = Vec::new();
    for record in &records {

        let chars: Vec<char> = record.springs.chars().collect();
        let mut prev_q = false;
        let mut range: Range<usize> = 0..record.springs.len();
        let mut ranges: Vec<Range<usize>> = Vec::new();
        for i in 0..record.springs.len() {
            if chars[i] == '?' {
                if prev_q == false {
                    range.start = i;
                }
                prev_q = true;
            } else {
                if prev_q == true {
                    range.end = i;
                }
                prev_q = true;
            }
            ranges.push(range.clone());
        }
        qs.push(ranges.clone());
    }

    println!("q ranges:");
    for q in qs {
        println!("{:?}", q);
    }


}

// fn get_options(springs: &String) -> Vec<String> {
//     let unknowns: Vec<String>
//
// }

fn test_record(springs: &String, rules: &Vec<u8>) -> bool {
    let springs: Vec<_> = springs.split('.').collect();

    let lengths: Vec<_> = springs.into_iter().map(|x| x.len() as u8).collect();

    &lengths == rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_record_pass() {
        let springs = String::from("#.#.###");
        let rules: Vec<u8> = vec![1, 1, 3];

        assert_eq!(test_record(&springs, &rules), true)
    }

    #[test]
    fn test_test_record_fail() {
        let springs = String::from("#.#.###");
        let rules: Vec<u8> = vec![1, 1, 4];

        assert_eq!(test_record(&springs, &rules), false)
    }
}

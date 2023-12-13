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
        Record { rules, springs: spring_str.to_string() }
    }
}

fn main() {
    let contents = std::fs::read_to_string("example.txt").unwrap();
    let records: Vec<Record> = contents.lines().map(|l| Record::from(l)).collect();

    println!("{:?}", records);
}

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
        let rules: Vec<u8> = vec![1,1,3];

        assert_eq!(test_record(&springs, &rules), true)
    }

    #[test]
    fn test_test_record_fail() {
        let springs = String::from("#.#.###");
        let rules: Vec<u8> = vec![1,1,4];

        assert_eq!(test_record(&springs, &rules), false)
    }
}

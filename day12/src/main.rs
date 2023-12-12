#[derive(Debug)]
struct Record {
    rules: Vec<u8>,
    springs: Vec<char>,
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let (spring_str, rules_str) = value.split_once(" ").unwrap();
        let springs: Vec<char> = spring_str.chars().collect();
        let rules: Vec<u8> = rules_str
            .split(",")
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        Record { rules, springs }
    }
}

fn main() {
    let contents = std::fs::read_to_string("example.txt").unwrap();
    let records: Vec<Record> = contents.lines().map(|l| Record::from(l)).collect();

    println!("{:?}", records);
    
}

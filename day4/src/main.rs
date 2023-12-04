#[derive(Debug, PartialEq)]
struct Card {
    num: u32,
    winners: Vec<u32>,
    drawn: Vec<u32>,
}
impl Card {
    fn from_string(line: &str) -> Self {
        let (prefix, drawn_string) = line.split_once("|").unwrap();
        let drawn: Vec<u32> = drawn_string
            .split(" ")
            .filter(|&c| c != "")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let (card_string, winner_string) = prefix.split_once(":").unwrap();
        let winners: Vec<u32> = winner_string
            .split(" ")
            .filter(|&c| c != "")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let card_vec: Vec<&str> = card_string.split(" ").collect();

        Card {
            num: card_vec.last().unwrap().parse::<u32>().unwrap(),
            winners,
            drawn,
        }
    }

    fn score(self) -> u32 {
        let mut score = 0;
        let mut win = false;
        for num in self.drawn {
            if self.winners.contains(&num) {
                if win == false {
                    score = 1;
                    win = true;
                } else {
                    score *= 2;
                }
            }
        }

        score
    }
}

fn main() {
    println!("Hello, world!");

    let content = std::fs::read_to_string("input.txt").unwrap();
    let lines = content.lines();

    let mut sum = 0;
    for line in lines {
        let card = Card::from_string(line);
        sum += card.score();
    }

    println!("Part 1 sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_from_string() {
        let card_string = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_string(card_string);

        let expected_card = Card {
            num: 1,
            winners: vec![41, 48, 83, 86, 17],
            drawn: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        assert_eq!(card, expected_card)
    }

    #[test]
    fn test_card1_score() {
        let card_string = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_string(card_string);
        let score = card.score();

        assert_eq!(score, 8)
    }
}

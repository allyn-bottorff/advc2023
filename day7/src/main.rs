use std::{collections::HashMap, cmp::Ordering};

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();

    let mut plays: Vec<Play> = contents
        .lines()
        .map(|x| Play::from_str(x))
        .collect();

    // println!("{:#?}", plays);

    plays.sort();

    // println!("{:#?}", plays);

    let mut sum: u32 = 0;

    for (i, play) in plays.into_iter().enumerate() {
        sum += play.bid * u32::try_from(i+1).unwrap();
    }

    println!("sum of bets: {}", sum);



}

#[derive(Debug,Eq)]
struct Play {
    hand: String,
    bid: u32,
    score: u8,
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => {
                let self_chars = &self.hand.chars().collect::<Vec<char>>();
                let other_chars = &other.hand.chars().collect::<Vec<char>>();
                for i in 0..self.hand.len() {
                    match &card_val(&self_chars[i]).cmp(&card_val(&other_chars[i])) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                    };
                }
                return Ordering::Equal
            },
        };
    }

}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Play {
    fn from_str(s: &str) -> Self {
        let (hand, bet) = s.split_once(" ").unwrap();

        let mut play = Play {
            hand: hand.to_string(),
            bid: bet.parse::<u32>().unwrap(),
            score: 0,
        };
        play.score_hand();
        play
    }
    /// Score Hands
    /// High Card:      1
    /// One Pair:       2
    /// Two Pair:       3
    /// Triple:         4
    /// Full House:     5
    /// Quad:           6
    /// Quint:          7
    fn score_hand(&mut self) {
        let hand: Vec<char> = self.hand.chars().collect();
        let mut score = 1;
        let mut found_triples = 0;
        let mut found_doubles = 0;

        let mut card_map = HashMap::from([
            ('2', 0),
            ('3', 0),
            ('4', 0),
            ('5', 0),
            ('6', 0),
            ('7', 0),
            ('8', 0),
            ('9', 0),
            ('T', 0),
            ('J', 0),
            ('K', 0),
            ('Q', 0),
            ('A', 0),
        ]);

        for i in hand {
            let count = card_map.get(&i).unwrap();
            card_map.insert(i, count + 1);
        }

        for q in card_map {
            if q.1 == 5 {
                score = 7;
                break;
            }
            if q.1 == 4 {
                score = 6;
                break;
            }
            if q.1 == 3 {
                found_triples += 1;
            }
            if q.1 == 2 {
                found_doubles += 1;
            }
        }
        if found_triples == 1 {
            if found_doubles == 1 {
                score = 5;
            } else {
                score = 4;
            }
        } else {
            if found_doubles == 2 {
                score = 3
            }
            if found_doubles == 1 {
                score = 2
            }
        }

        self.score = score;
    }
}

fn card_val(card: &char) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("unexpected card char"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_hand_1() {
        let s = "32T3K 0";
        let play = Play::from_str(s);
        assert_eq!(2, play.score)
    }

    #[test]
    fn test_score_hand_2() {
        let s = "KK677 12";
        let play = Play::from_str(s);
        assert_eq!(3, play.score)
    }
    #[test]
    fn test_score_hand_3() {
        let s = "QQQJA 3";
        let play = Play::from_str(s);
        assert_eq!(4, play.score)
    }
    #[test]
    fn test_score_hand_4() {
        let s = "QQQJA 5";
        let play = Play::from_str(s);
        assert_eq!(4, play.score)
    }
}

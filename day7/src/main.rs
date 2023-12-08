use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();

    let mut plays: Vec<Play> = contents.lines().map(|x| Play::from_str(x)).collect();

    // println!("{:#?}", plays);

    plays.sort();

    println!("{:#?}", plays);

    let mut sum: u32 = 0;

    for (i, play) in plays.into_iter().enumerate() {
        sum += play.bid * u32::try_from(i + 1).unwrap();
    }

    println!("sum of bets: {}", sum);
}

#[derive(Debug, Eq)]
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
                return Ordering::Equal;
            }
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

        let jokers = card_map.get(&'J').unwrap().clone();

        let score = match jokers {
            5 => 7,
            4 => 7,
            3 => {
                let mut temp_count = 6;
                for q in &card_map {
                    if *q.0 != 'J' {
                        if *q.1 == 2 {
                            temp_count = 7;
                            break;
                        }
                    }
                }
                temp_count
            }
            2 => {
                let mut temp_score = 4;
                for q in &card_map {
                    if *q.0 != 'J' {
                        if *q.1 == 3 {
                            temp_score = 7;
                            break;
                        }
                        if *q.1 == 2 {
                            temp_score = 6;
                            break;
                        }
                    }
                }
                temp_score
            }
            1 => {
                let mut temp_score = 2;
                let mut found_double = false;
                for q in &card_map {
                    if *q.0 != 'J' {
                        if *q.1 == 4 {
                            temp_score = 7;
                            break;
                        }
                        if *q.1 == 3 {
                            temp_score = 6;
                            break;
                        }
                        if *q.1 == 2 {
                            if !found_double {
                                temp_score = 4;
                                found_double = true;
                            } else {
                                temp_score = 5;
                                break;
                            }
                        }
                    }
                }
                temp_score
            }
            0 => {
                let mut temp_score = 1;
                let mut found_double = false;
                let mut found_triple = false;
                for q in &card_map {
                    if *q.1 == 5 {
                        temp_score = 7;
                        break;
                    }
                    if *q.1 == 4 {
                        temp_score = 6;
                        break;
                    }
                    if *q.1 == 3 {
                        temp_score = 4;
                        found_triple = true;
                        if found_double == true {
                            temp_score = 5;
                            break;
                        }
                    }
                    if *q.1 == 2 {
                        if found_triple == true {
                            temp_score = 5;
                            break;
                        }
                        if !found_double {
                            temp_score = 2;
                            found_double = true;
                        } else {
                            temp_score = 3;
                            break;
                        }
                    }
                }
                temp_score
            }
            _ => panic!("too many jokers"),
        };

        self.score = score;
    }
}

fn card_val(card: &char) -> u8 {
    match card {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
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
        assert_eq!(6, play.score)
    }
    #[test]
    fn test_score_hand_4() {
        let s = "QQJJA 5";
        let play = Play::from_str(s);
        assert_eq!(6, play.score)
    }
    #[test]
    fn test_score_hand_5() {
        let s = "JJ2K8 5";
        let play = Play::from_str(s);
        assert_eq!(4, play.score)
    }
    #[test]
    fn test_score_hand_6() {
        let s = "KJT64 2";
        let play = Play::from_str(s);
        assert_eq!(2, play.score)
    }
    #[test]
    fn test_score_hand_7() {
        let s = "9AJ89 2";
        let play = Play::from_str(s);
        assert_eq!(4, play.score)
    }
    #[test]
    fn test_score_hand_8() {
        let s = "J8899 2";
        let play = Play::from_str(s);
        assert_eq!(5, play.score)
    }
    #[test]
    fn test_score_hand_9() {
        let s = "88899 2";
        let play = Play::from_str(s);
        assert_eq!(5, play.score)
    }

    #[test]
    fn test_functional_pt2() {
        let contents = std::fs::read_to_string("example.txt").unwrap();
        let mut plays: Vec<Play> = contents.lines().map(|x| Play::from_str(x)).collect();

        plays.sort();
        let mut sum: u32 = 0;

        for (i, play) in plays.into_iter().enumerate() {
            sum += play.bid * u32::try_from(i + 1).unwrap();
        }
        assert_eq!(5905, sum)
    }
}

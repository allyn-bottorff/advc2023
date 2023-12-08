use std::collections::HashMap;

fn main() {
    let contents = std::fs::read_to_string("example.txt").unwrap();

    let plays: Vec<(&str, u32)> = contents
        .lines()
        .map(|x| x.split_once(" ").unwrap())
        .map(|x| (x.0, x.1.parse::<u32>().unwrap()))
        .collect();

    println!("{:?}", plays);
    let _score = score_hand("32T3K");
}

/// Score Hands
/// High Card:      1
/// One Pair:       2
/// Two Pair:       3
/// Triple:         4
/// Quad:           5
/// Full House:     6
/// Quint:          7
fn score_hand(hand: &str) -> u8 {
    let hand: Vec<char> = hand.chars().collect();
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

    println!("quantities: {:?}", card_map);
    for q in card_map {
        if q.1 == 5 {
            score = 7;
            break;
        }
        if q.1 == 4 {
            score = 5;
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
        score = 6;
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

    score
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
    use crate::score_hand;

    #[test]
    fn test_score_hand_1() {
        let hand = "32T3K";
        let score = score_hand(hand);
        assert_eq!(2, score)
    }

    #[test]
    fn test_score_hand_2() {
        let hand = "KK677";
        let score = score_hand(hand);
        assert_eq!(3, score)
    }
    #[test]
    fn test_score_hand_3() {
        let hand = "QQQJA";
        let score = score_hand(hand);
        assert_eq!(4, score)
    }
    #[test]
    fn test_score_hand_4() {
        let hand = "QQQJA";
        let score = score_hand(hand);
        assert_eq!(4, score)
    }
}

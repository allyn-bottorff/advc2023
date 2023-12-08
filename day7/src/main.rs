fn main() {
    let contents = std::fs::read_to_string("example.txt").unwrap();

    let plays: Vec<(&str, u32)> = contents
        .lines()
        .map(|x| x.split_once(" ").unwrap())
        .map(|x| (x.0, x.1.parse::<u32>().unwrap()))
        .collect();

    println!("{:?}", plays);
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

    let mut quantities: Vec<u8> = vec![0; 13];
    for i in 0..hand.len() {
        for j in 0..hand.len() {
            if hand[i] == hand[j] {
                let index = match hand[i] {
                    '2' => 0,
                    '3' => 1,
                    '4' => 2,
                    '5' => 3,
                    '6' => 4,
                    '7' => 5,
                    '8' => 6,
                    '9' => 7,
                    'T' => 8,
                    'J' => 9,
                    'Q' => 10,
                    'K' => 11,
                    'A' => 12,
                    _ => panic!("unexpected card char"),
                };
                quantities[index] += 1;
            }
        }
    }
    for q in quantities {
        if q == 5 {
            score = 7;
                break
        }
        if q == 4 {
            score = 5;
            break
        }
        if q == 3 {
            found_triples += 1;
        }
        if q == 2 {
            found_doubles += 1;
        }

        if found_doubles == 2 {
            score = 3
        }
        if found_triples ==1 && found_doubles == 1{
            score = 6
        }
        if found_doubles == 1{
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
}

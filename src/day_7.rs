struct Hand {
    points: usize,
    cards: String,
}

struct HandResult {
    hand: Hand,
    value: Vec<usize>,
    m: Match,
}

const CARD_TYPES_TASK_1: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const CARD_TYPES_TASK_2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(Debug, Clone, Copy, PartialEq)]
enum Match {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAkind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Match {
    fn from_cards_task_1(cards: &str) -> Self {
        let mut first_match = vec![];
        let mut second_match = vec![];
        let mut third_match = vec![];
        let mut fourth_match = vec![];

        for (i, card) in cards.chars().enumerate() {
            if i == 0 {
                first_match.push(card);
                continue;
            }
            if first_match.iter().any(|f| f == &card) {
                first_match.push(card)
            } else if second_match.len() == 0 || second_match.iter().any(|s| s == &card) {
                second_match.push(card);
            } else if third_match.len() == 0 || third_match.iter().any(|t| t == &card) {
                third_match.push(card);
            } else if fourth_match.len() == 0 || fourth_match.iter().any(|f| f == &card) {
                fourth_match.push(card);
            }
        }

        let matches = vec![
            first_match.clone(),
            second_match.clone(),
            third_match.clone(),
            fourth_match.clone(),
        ];

        if first_match.len() == 5 {
            Match::FiveOfAKind
        } else if first_match.len() == 4 || second_match.len() == 4 {
            Match::FourOfAKind
        } else if (first_match.len() == 3 && second_match.len() == 2)
            || (second_match.len() == 3 && first_match.len() == 2)
        {
            Match::FullHouse
        } else if (first_match.len() == 3 || second_match.len() == 3 || third_match.len() == 3) {
            Match::ThreeOfAkind
        } else if matches.iter().filter(|m| m.len() == 2).count() == 2 {
            Match::TwoPair
        } else if matches.iter().any(|m| m.len() == 2) {
            Match::OnePair
        } else {
            Match::HighCard
        }
    }
    fn from_cards_task_2(cards: &str) -> Self {
        let mut first_match = vec![];
        let mut second_match = vec![];
        let mut third_match = vec![];
        let mut fourth_match = vec![];
        let mut fifth_match = vec![];

        let filtered_cards: String = cards.chars().filter(|c| c != &'J').collect();

        for (i, card) in filtered_cards.chars().enumerate() {
            if i == 0 {
                first_match.push(card);
                continue;
            }
            if first_match.iter().any(|f| f == &card) {
                first_match.push(card)
            } else if second_match.len() == 0 || second_match.iter().any(|s| s == &card) {
                second_match.push(card);
            } else if third_match.len() == 0 || third_match.iter().any(|t| t == &card) {
                third_match.push(card);
            } else if fourth_match.len() == 0 || fourth_match.iter().any(|f| f == &card) {
                fourth_match.push(card);
            } else if fifth_match.len() == 0 || fifth_match.iter().any(|f| f == &card) {
                fifth_match.push(card);
            }
        }

        let matches = vec![
            first_match.clone(),
            second_match.clone(),
            third_match.clone(),
            fourth_match.clone(),
            fifth_match.clone(),
        ];

        let joker_count = cards.chars().filter(|c| c == &'J').count();
        let pair_amount = matches.iter().filter(|m| m.len() == 2).count();

        if first_match.len() == 5 || first_match.len() + joker_count == 5 {
            Match::FiveOfAKind
        } else if first_match.len() == 4
            || first_match.len() + joker_count == 4
            || second_match.len() == 4
            || second_match.len() + joker_count == 4
        {
            Match::FourOfAKind
        } else if (first_match.len() == 3 && second_match.len() == 2)
            || (second_match.len() == 3 && first_match.len() == 2)
            || first_match.len() + joker_count == 3 && second_match.len() == 2
            || first_match.len() == 3 && second_match.len() + joker_count == 2
            || second_match.len() + joker_count == 3 && first_match.len() == 2
            || second_match.len() == 3 && second_match.len() + joker_count == 2
        {
            Match::FullHouse
        } else if first_match.len() == 3
            || second_match.len() == 3
            || third_match.len() == 3
            || first_match.len() + joker_count == 3
            || second_match.len() + joker_count == 3
            || third_match.len() + joker_count == 3
            || fourth_match.len() + joker_count == 3
        {
            Match::ThreeOfAkind
        } else if pair_amount >= 2 || (pair_amount >= 1 && joker_count >= 1) || joker_count >= 2 {
            Match::TwoPair
        } else if pair_amount >= 1 || joker_count >= 1 {
            Match::OnePair
        } else {
            Match::HighCard
        }
    }
}

fn task(input: &str, n: usize) -> usize {
    let mut hands = vec![];
    let mut hand_results = vec![];

    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let hand = Hand {
            cards: split[0].to_string(),
            points: split[1].parse().unwrap(),
        };
        hands.push(hand);
    }

    // calculate enum

    for hand in hands {
        let m = match n {
            1 => Match::from_cards_task_1(&hand.cards),
            _ => Match::from_cards_task_2(&hand.cards),
        };

        if hand.cards.contains('J') && m == Match::OnePair {
            println!("{}: {m:?}", hand.cards);
        }

        let mut hand_num_value = vec![];

        for c in hand.cards.chars() {
            let card_types = match n {
                1 => CARD_TYPES_TASK_1.to_vec(),
                _ => CARD_TYPES_TASK_2.to_vec(),
            };

            hand_num_value.push(card_types.iter().position(|ct| ct == &c).unwrap())
        }

        let hr = HandResult {
            m,
            hand,
            value: hand_num_value,
        };

        hand_results.push(hr);
    }

    hand_results.sort_by(|a, b| a.value.cmp(&b.value));
    hand_results.sort_by(|a, b| (a.m as u8).cmp(&(b.m as u8)));

    let mut solution = 0;

    for (i, hr) in hand_results.iter().enumerate() {
        let rank = hand_results.len() - i;

        solution += hr.hand.points * rank;
    }

    solution
}

fn task_1(input: &str) -> usize {
    task(input, 1)
}
fn task_2(input: &str) -> usize {
    task(input, 2)
}

#[cfg(test)]
mod tests {
    use super::{task_1, task_2};

    fn input() -> String {
        std::fs::read_to_string("./data/day_7.txt").unwrap()
    }
    fn sample_input() -> String {
        std::fs::read_to_string("./data/day_7_sample.txt").unwrap()
    }

    #[test]
    fn test_day_7_task_1() {
        let sample_result = task_1(&sample_input());
        assert_eq!(sample_result, 6440);

        let result = task_1(&input());
        println!("the solution for day 7, task 1 is: {result}",);
    }

    #[test]
    fn test_day_7_task_2() {
        let sample_result = task_2(&sample_input());
        assert_eq!(sample_result, 5905);

        let result = task_2(&input());
        println!("the solution for day 7, task 2 is: {result}",);
    }
}

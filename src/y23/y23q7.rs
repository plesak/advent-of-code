use std::collections::HashSet;

pub fn y23q7q1(input: Vec<String>) -> String {

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Hand {
        hand_type: HandType,
        card_values: [u8; 5],
        cards: String,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
    enum HandType {
        #[default]
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    impl Hand {
        fn new(card_hand: String) -> Self {
            assert_eq!(card_hand.len(), 5);

            let mut vals:[u8; 5] = [0; 5];
            for (i, card) in card_hand.chars().enumerate() {
                let card_val:u8 = match card {
                    card if card.is_ascii_digit() => {
                        card.to_digit(10).unwrap() as u8
                    },
                    'T' => { 10 },
                    'J' => { 11 },
                    'Q' => { 12 },
                    'K' => { 13 },
                    'A' => { 14 },
                    _ => { unreachable!("unknown card: {}", card); },
                };
                vals[i] = card_val;
            }

            let unique_cards: HashSet<_> = card_hand.chars().collect();

            let hand_type:HandType = match unique_cards.len() {
                1 => HandType::FiveOfAKind,
                2 => {
                    // Check if it's Four of a Kind or Full House
                    if vals.iter().any(|&card| vals.iter().filter(|&&c| c == card).count() == 4) {
                        HandType::FourOfAKind
                    } else {
                        HandType::FullHouse
                    }
                }
                3 => {
                    // Check if it's Three of a Kind or Two Pairs
                    if vals.iter().any(|&card| vals.iter().filter(|&&c| c == card).count() == 3) {
                        HandType::ThreeOfAKind
                    } else {
                        HandType::TwoPair
                    }
                }
                4 => HandType::OnePair,
                5 => HandType::HighCard,
                _ => unreachable!("Invalid number of cards"),
            };

            Hand {
                hand_type,
                card_values: vals,
                cards: card_hand,
            }
        }
    }

    let mut hands:Vec<(Hand, usize)> = vec!();
    for ln in input {
        // 32T3K 765
        let hand = Hand::new(ln.chars().take(5).collect::<String>());
        let bet = ln.chars().skip(6).collect::<String>().parse::<usize>().unwrap();
        hands.push((hand, bet));
    }

    hands.sort();

    let mut sm = 0;
    for i in 0..hands.len() {
        let bet = hands[i].1;
        sm += bet * (i+1);
    }

    sm.to_string()
}

pub fn y23q7q2(input: Vec<String>) -> String {

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Hand {
        hand_type: HandType,
        card_values: [u8; 5],
        cards: String,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
    enum HandType {
        #[default]
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    impl Hand {
        fn new(card_hand: String) -> Self {
            assert_eq!(card_hand.len(), 5);

            let mut vals:[u8; 5] = [0; 5];
            for (i, card) in card_hand.chars().enumerate() {
                let card_val:u8 = match card {
                    card if card.is_ascii_digit() => {
                        card.to_digit(10).unwrap() as u8
                    },
                    'T' => { 10 },
                    'J' => { 1 },
                    'Q' => { 12 },
                    'K' => { 13 },
                    'A' => { 14 },
                    _ => { unreachable!("unknown card: {}", card); },
                };
                vals[i] = card_val;
            }

            let jokerless:String = card_hand.chars().filter(|c| !c.eq(&'J')).collect::<String>();
            let num_jokers = 5 - jokerless.len();
            let unique_cards: HashSet<_> = jokerless.chars().collect();

            let hand_type:HandType = match unique_cards.len() {
                // can only happen for "JJJJJ"
                0 => HandType::FiveOfAKind,
                // "XXXXX", "XXXXJ", "XXXJJ", "XXJJJ", "XJJJJ"
                1 => HandType::FiveOfAKind,
                2 => {
                    if vals.iter().any(|&card| vals.iter().filter(|&&c| c == card).count() == 4) {
                        // "XXXXY"
                        HandType::FourOfAKind
                    } else {
                        if num_jokers == 2 ||
                            num_jokers == 3 ||
                            (num_jokers == 1 &&
                                vals.iter().any(|&card| vals.iter().filter(|&&c| c == card).count() == 3)) {
                            // "XXXYJ", "XXYJJ", "XYJJJ"
                            HandType::FourOfAKind
                        } else {
                            // "XXXYY", "XXYYJ"
                            HandType::FullHouse
                        }
                    }
                }
                3 => {
                    if num_jokers > 0 ||
                        vals.iter().any(|&card| vals.iter().filter(|&&c| c == card).count() == 3) {
                        // "XXXYZ", "XXYZJ", "XYZJJ"
                        HandType::ThreeOfAKind
                    } else {
                        // "XXYYZ"
                        HandType::TwoPair
                    }
                }
                // "XXYZA", "XYZAJ"
                4 => HandType::OnePair,
                // "XYZAB"
                5 => HandType::HighCard,
                _ => unreachable!("Invalid number of cards"),
            };

            Hand {
                hand_type,
                card_values: vals,
                cards: card_hand,
            }
        }
    }

    let mut hands:Vec<(Hand, usize)> = vec!();
    for ln in input {
        // 32T3K 765
        let hand = Hand::new(ln.chars().take(5).collect::<String>());
        let bet = ln.chars().skip(6).collect::<String>().parse::<usize>().unwrap();
        hands.push((hand, bet));
    }

    hands.sort();

    let mut sm = 0;
    for i in 0..hands.len() {
        let bet = hands[i].1;
        sm += bet * (i+1);
    }

    sm.to_string()
}
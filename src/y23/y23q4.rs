use std::cmp::min;

pub fn y23q4q1(input: Vec<String>) -> String {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // 2^(4-1) + 2^(2-1) + 2^(2-1) = 12

    let mut sm = 0;

    for ln in input {
        let winning_nums:Vec<&str> = ln.split([':', '|']).nth(1)
            .unwrap_or("").split_whitespace().collect();
        let test_nums:Vec<&str> = ln.split('|').nth(1)
            .unwrap_or("").split_whitespace().collect();

        let mut wins = 0;
        for n in test_nums {
            if winning_nums.contains(&n) {wins += 1;}
        }
        if wins > 0 {sm += 1 << (wins-1);}
    }

    sm.to_string()
}

pub fn y23q4q2(input: Vec<String>) -> String {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    // C1 wins 4 copies (C2 C3 C4 C5)
    // 2xC2 win 2 copies each (2xC3 2xC4)
    // 4xC3 win 2 copies each (4xC4 4xC5)
    // 8xC4 win 1 copy each (8xC5)
    // 14xC5 wins 0 copies
    // C6 wins 0 copies
    // total 30 cards

    let num_rows = input.len();
    let mut num_cards:Vec<usize> = Vec::with_capacity(num_rows);
    for _ in 0..num_rows {num_cards.push(1);}

    for (i, ln) in input.iter().enumerate() {
        let winning_nums:Vec<&str> = ln.split([':', '|']).nth(1)
            .unwrap_or("").split_whitespace().collect();
        let test_nums:Vec<&str> = ln.split('|').nth(1)
            .unwrap_or("").split_whitespace().collect();

        let mut wins = 0;
        for n in test_nums {
            if winning_nums.contains(&n) {wins += 1;}
        }

        let curr_cards = num_cards[i];
        for j in i+1..min(i+1+wins, num_rows) {
            num_cards[j] += 1*curr_cards;
        }
    }

    num_cards.iter().sum::<usize>().to_string()
}
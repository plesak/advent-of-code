use std::cmp::max;

pub fn y23q2q1(input: Vec<String>) -> String {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // true

    const MAX_RED:usize = 12;
    const MAX_GREEN:usize = 13;
    const MAX_BLUE:usize = 14;

    let mut sm = 0;

    for (i, ln) in input.iter().enumerate() {

        let mut valid = true;

        let gameround:Vec<String> = ln.split(&[':', ';', ','][..]).skip(1)
            .map(|s| s.trim().to_owned()).collect::<Vec<String>>();

        for round in gameround {
            let num = round.split_whitespace()
                .find(|word| {word.chars().all(|c| {c.is_digit(10)})})
                .and_then(|word| word.parse::<usize>().ok())
                .unwrap_or(0);

            if round.contains("red") && num > MAX_RED { valid = false; break; }
            else if round.contains("green") && num > MAX_GREEN { valid = false; break; }
            else if round.contains("blue") && num > MAX_BLUE { valid = false; break; }
        }

        if valid {
            sm += i+1;
        }
    }

    sm.to_string()
}

pub fn y23q2q2(input: Vec<String>) -> String {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // 4r * 2g * 6b = 48

    let mut sm = 0;

    for ln in input {

        let mut game_mins = [0, 0, 0]; // red min, green min, blue min

        let gameround:Vec<String> = ln.split(&[':', ';', ','][..]).skip(1)
            .map(|s| s.trim().to_owned()).collect::<Vec<String>>();

        for round in gameround {
            let num = round.split_whitespace()
                .find(|word| { word.chars().all(|c| { c.is_digit(10) }) })
                .and_then(|word| word.parse::<usize>().ok())
                .unwrap_or(0);

            if round.contains("red") { game_mins[0] = max(game_mins[0], num); }
            else if round.contains("green") { game_mins[1] = max(game_mins[1], num); }
            else if round.contains("blue") { game_mins[2] = max(game_mins[2], num); }
        }
        sm += game_mins[0]*game_mins[1]*game_mins[2];
    }
    sm.to_string()
}
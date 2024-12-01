pub fn y23q1q1(input: Vec<String>) -> String {
    // 1abc2
    // pqr3stu8vwx
    // 12 + 38 = 50

    let mut sm = 0;
    for ln in input {
        let digits:Vec<char> = ln.chars().filter(|ch| ch.is_ascii_digit()).collect();
        let first_digit = digits.first().unwrap().to_digit(10).unwrap();
        let last_digit = digits.last().unwrap().to_digit(10).unwrap();
        sm += first_digit*10 + last_digit;
    }
    sm.to_string()
}

pub fn y23q1q2(input: Vec<String>) -> String {
    // two1nine
    // eightwothree
    // 29 + 83 = 112

    let mut sm = 0;
    for ln in input {

        let mut first_digit:u32 = 0;
        let mut last_digit:u32 = 0;

        for (i, c) in ln.char_indices() {
            match c {
                c if c.is_ascii_digit() => {
                    first_digit = c.to_digit(10).unwrap();
                    break;
                },
                'o' => {
                    if ln[i..].starts_with("one") {
                        first_digit = 1; break;
                    }
                }
                't' => {
                    if ln[i..].starts_with("two") {
                        first_digit = 2; break;
                    }
                    if ln[i..].starts_with("three") {
                        first_digit = 3; break;
                    }
                }
                'f' => {
                    if ln[i..].starts_with("four") {
                        first_digit = 4; break;
                    }
                    if ln[i..].starts_with("five") {
                        first_digit = 5; break;
                    }
                }
                's' => {
                    if ln[i..].starts_with("six") {
                        first_digit = 6; break;
                    }
                    if ln[i..].starts_with("seven") {
                        first_digit = 7; break;
                    }
                }
                'e' => {
                    if ln[i..].starts_with("eight") {
                        first_digit = 8; break;
                    }
                }
                'n' => {
                    if ln[i..].starts_with("nine") {
                        first_digit = 9; break;
                    }
                }
                _ => {}
            }
        }

        let lnr = ln.chars().rev().collect::<String>();
        for (i, c) in lnr.char_indices() {
            match c {
                c if c.is_ascii_digit() => {
                    last_digit = c.to_digit(10).unwrap();
                    break;
                }
                'e'=> {
                    if lnr[i..].starts_with("eno") {
                        last_digit = 1; break;
                    }
                    if lnr[i..].starts_with("eerht") {
                        last_digit = 3; break;
                    }
                    if lnr[i..].starts_with("evif") {
                        last_digit = 5; break;
                    }
                    if lnr[i..].starts_with("enin") {
                        last_digit = 9; break;
                    }
                }
                'o' => {
                    if lnr[i..].starts_with("owt") {
                        last_digit = 2; break;
                    }
                }
                'r' => {
                    if lnr[i..].starts_with("ruof") {
                        last_digit = 4; break;
                    }
                }
                'x' => {
                    if lnr[i..].starts_with("xis") {
                        last_digit = 6; break;
                    }
                }
                'n' => {
                    if lnr[i..].starts_with("neves") {
                        last_digit = 7; break;
                    }
                }
                't' => {
                    if lnr[i..].starts_with("thgie") {
                        last_digit = 8; break;
                    }
                }
                _ => {}
            }
        }

        // println!("{} {} {}", lnr, first_digit, last_digit);
        sm += 10*first_digit + last_digit;

    }

    sm.to_string()
}
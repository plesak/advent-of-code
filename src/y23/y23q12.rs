use std::collections::{HashMap, VecDeque};

pub fn y23q12q1(input:Vec<String>) -> String {
    // ?###???????? 3,2,1   --> 10
    // .###.##.#... | .###.##..#.. | .###.##...#. | .###.##....#
    // .###..##.#.. | .###..##..#. | .###..##...#
    // .###...##.#. | .###...##..#
    // .###....##.#

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Pattern(String, VecDeque<usize>);

    fn parse_ln(inp:String) -> Pattern {
        let patt:String = inp.split(' ').take(1).collect::<String>()
            .trim_start_matches('.').trim_end_matches('.').to_string();
        let cnts:VecDeque<usize> = inp
            .split(' ').skip(1).take(1).collect::<String>()
            .split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        Pattern(patt, cnts)
    }

    let mut memo:HashMap<Pattern, usize> = HashMap::new();
    memo.insert(Pattern("?".to_string(), VecDeque::from([1])), 1);    // #
    memo.insert(Pattern("??".to_string(), VecDeque::from([1])), 2);   // #. | .#
    memo.insert(Pattern("??".to_string(), VecDeque::from([2])), 1);   // ##

    fn cnt_solutions(inp:Pattern, memo:&mut HashMap<Pattern, usize>) -> usize {

        // println!("solving for {:?}", inp);

        // cut away any '.'
        let mut s = inp.0.clone();
        s = s.trim_start_matches('.').trim_end_matches('.').to_string();

        let mut cnts = inp.1.clone();
        // base case
        if cnts.iter().sum::<usize>() > s.len() {
            // println!("returning for too short string {:?}", inp);
            return 0;
        }
        if s.len() == 0 {
            // println!("retuning from base case {:?}", inp);
            //                        "" []      "" [1]
            return if cnts.len() == 0 { 1 } else { 0 }
        }
        if cnts.len() == 0 {
            // println!("retuning from base case {:?}", inp);
            //                                                      "?.?" []   "?#?" []
            return if s.chars().all(|c| c == '?' || c == '.') { 1 } else { 0 }
        }

        if let Some(&count) = memo.get(&Pattern(s.clone(), cnts.clone())) {
            return count;
        }

        // main logic for partially figuring out the start of a pattern
        let x = cnts.pop_front().unwrap();

        // trim MORE eg "?.???" [2] should trim away "?."
        // but "#.???" [2] should return 0
        while s.chars().take_while(|&c| c == '?' || c == '#').count() < x {
            // println!("TRIGGERED");
            if s.chars().take_while(|&c| c == '?').count()
                < s.chars().take_while(|&c| c == '?' || c == '#').count() {
                return 0;
            }
            s = s.trim_start_matches(|c| c == '?' || c == '#')
                .trim_start_matches('.').to_string();
            if s.len() < x { return 0; }
        }

        let mut skip_counter = 0;
        let mut prefix_valid = false;
        let mut s_mod = s.clone();
        while !prefix_valid {
            let pref = s_mod.chars().take(x).collect::<String>();
            let sep = s_mod.chars().nth(x).unwrap_or('.');
            if pref.chars().all(|c| c == '?' || c == '#')
                && (sep == '?' || sep == '.') {
                s_mod = s_mod.chars().skip(x+1).collect::<String>();
                prefix_valid = true;
            } else if !(s_mod.chars().nth(0).unwrap() == '#') {
                skip_counter += 1;
                s_mod = s_mod.chars().skip(1).collect::<String>();
            } else {
                // eg ## 1
                return 0;
            }
        }

        // eg for "???" [1]                         // 3
        // count( "?" [])     +   count( "??" [1] )
        // base case (+1)     +   count( "" [] )    +   count( "?" [1] )
        //            +1                    +1                      +1
        let mut c1 = cnt_solutions(
            Pattern(s_mod.trim_start_matches('.').trim_end_matches('.').to_string(),
                    cnts.clone()), memo);
        // if c1 == 1 { println!("found single solution for {}, {:?}", s_mod, cnts); }

        // println!("considering to branch with {:?} vs {:?} & skip {}", s, s_mod, skip_counter);
        if s.chars().skip(skip_counter).nth(0).unwrap_or('.') == '?'
            && s.len()-skip_counter == s_mod.len()+x+1 {
            // println!("branch2 with {:?} vs {:?}", s, s_mod);
            let c2 = cnt_solutions(
                Pattern(s.chars().skip(skip_counter+1).collect::<String>(),
                        inp.1.clone()), memo);
            // if c2 == 1 { println!("found single solution for {}, {:?}",
            //                       s.chars().skip(skip_counter+1).collect::<String>(),
            //                       inp.1.clone()); }
            c1 += c2;
        }

        // println!("returning {} for {:?}", c1, inp);
        memo.insert(inp, c1);
        c1
    }

    let mut running_total:usize = 0;
    for ln in input {
        running_total += cnt_solutions(parse_ln(ln), &mut memo);
    }
    // println!("memo: {:?}", memo);


    running_total.to_string()
}

pub fn y23q12q2(input:Vec<String>) -> String {
    // ?###???????? 3,2,1   --> 10
    // .###.##.#... | .###.##..#.. | .###.##...#. | .###.##....#
    // .###..##.#.. | .###..##..#. | .###..##...#
    // .###...##.#. | .###...##..#
    // .###....##.#

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Pattern(String, VecDeque<usize>);

    fn parse_ln(inp:String) -> Pattern {
        let mut patt:String = inp.split(' ').take(1).collect::<String>();
        // pattern is actually 5-fold
        patt.push('?');
        patt = patt.repeat(5);
        patt.pop();
        patt = patt.trim_start_matches('.').trim_end_matches('.').to_string();

        let cnts:VecDeque<usize> = inp
            .split(' ').skip(1).take(1).collect::<String>()
            .split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        let mut fn_cnts = VecDeque::with_capacity(cnts.len()*5);
        for _ in 0..5 {
            fn_cnts.extend(cnts.clone());
        }
        Pattern(patt, fn_cnts)
    }

    let mut memo:HashMap<Pattern, usize> = HashMap::new();
    memo.insert(Pattern("?".to_string(), VecDeque::from([1])), 1);    // #
    memo.insert(Pattern("??".to_string(), VecDeque::from([1])), 2);   // #. | .#
    memo.insert(Pattern("??".to_string(), VecDeque::from([2])), 1);   // ##

    fn cnt_solutions(inp:Pattern, memo:&mut HashMap<Pattern, usize>) -> usize {

        // println!("solving for {:?}", inp);

        // cut away any '.'
        let mut s = inp.0.clone();
        s = s.trim_start_matches('.').trim_end_matches('.').to_string();

        let mut cnts = inp.1.clone();
        // base case
        if cnts.iter().sum::<usize>() > s.len() {
            // println!("returning for too short string {:?}", inp);
            return 0;
        }
        if s.len() == 0 {
            // println!("retuning from base case {:?}", inp);
            //                        "" []      "" [1]
            return if cnts.len() == 0 { 1 } else { 0 }
        }
        if cnts.len() == 0 {
            // println!("retuning from base case {:?}", inp);
            //                                                      "?.?" []   "?#?" []
            return if s.chars().all(|c| c == '?' || c == '.') { 1 } else { 0 }
        }

        if let Some(&count) = memo.get(&Pattern(s.clone(), cnts.clone())) {
            return count;
        }

        // main logic for partially figuring out the start of a pattern
        let x = cnts.pop_front().unwrap();

        // trim MORE eg "?.???" [2] should trim away "?."
        // but "#.???" [2] should return 0
        while s.chars().take_while(|&c| c == '?' || c == '#').count() < x {
            // println!("TRIGGERED");
            if s.chars().take_while(|&c| c == '?').count()
                < s.chars().take_while(|&c| c == '?' || c == '#').count() {
                return 0;
            }
            s = s.trim_start_matches(|c| c == '?' || c == '#')
                .trim_start_matches('.').to_string();
            if s.len() < x { return 0; }
        }

        let mut skip_counter = 0;
        let mut prefix_valid = false;
        let mut s_mod = s.clone();
        while !prefix_valid {
            let pref = s_mod.chars().take(x).collect::<String>();
            let sep = s_mod.chars().nth(x).unwrap_or('.');
            if pref.chars().all(|c| c == '?' || c == '#')
                && (sep == '?' || sep == '.') {
                s_mod = s_mod.chars().skip(x+1).collect::<String>();
                prefix_valid = true;
            } else if !(s_mod.chars().nth(0).unwrap() == '#') {
                skip_counter += 1;
                s_mod = s_mod.chars().skip(1).collect::<String>();
            } else {
                // eg ## 1
                return 0;
            }
        }

        // eg for "???" [1]                         // 3
        // count( "?" [])     +   count( "??" [1] )
        // base case (+1)     +   count( "" [] )    +   count( "?" [1] )
        //            +1                    +1                      +1
        let mut c1 = cnt_solutions(
            Pattern(s_mod.trim_start_matches('.').trim_end_matches('.').to_string(),
                    cnts.clone()), memo);
        // if c1 == 1 { println!("found single solution for {}, {:?}", s_mod, cnts); }

        // println!("considering to branch with {:?} vs {:?} & skip {}", s, s_mod, skip_counter);
        if s.chars().skip(skip_counter).nth(0).unwrap_or('.') == '?'
            && s.len()-skip_counter == s_mod.len()+x+1 {
            // println!("branch2 with {:?} vs {:?}", s, s_mod);
            let c2 = cnt_solutions(
                Pattern(s.chars().skip(skip_counter+1).collect::<String>(),
                        inp.1.clone()), memo);
            // if c2 == 1 { println!("found single solution for {}, {:?}",
            //                       s.chars().skip(skip_counter+1).collect::<String>(),
            //                       inp.1.clone()); }
            c1 += c2;
        }

        // println!("returning {} for {:?}", c1, inp);
        memo.insert(inp, c1);
        c1
    }

    let mut running_total:usize = 0;
    for ln in input {
        running_total += cnt_solutions(parse_ln(ln), &mut memo);
    }
    // println!("memo: {:?}", memo);


    running_total.to_string()
}
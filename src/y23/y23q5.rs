use std::cmp::min;

pub fn y23q5q1(input: Vec<String>) -> String {
    // input range start, output range start,
    // output range start offset, range length
    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct Range(isize, isize, isize, isize);

    let mut conversions: [Vec<Range>; 7] = Default::default();

    let seeds:Vec<isize> = input[0].trim_start_matches("seeds: ")
        .split_whitespace().map(|s| s.parse::<isize>().unwrap_or(0)).collect();

    let mut curr_conversion_idx = 0;
    for line in &input[3..] {
        if line.contains("-to-") {
            curr_conversion_idx += 1;
        } else if line.is_empty() {
            continue;
        } else {
            // has 3 numbers separated by whitespace
            let rng:Vec<isize> = line.split_whitespace()
                .map(|s| s.parse::<isize>().unwrap_or(0)).collect();
            let rngg:Range = Range(rng[1], rng[0], rng[0]-rng[1], rng[2]);
            conversions[curr_conversion_idx].push(rngg);
        }
    }

    for i in 0..conversions.len() {
        conversions[i].sort();
        conversions[i].reverse();
    }

    let mut seed_locations:Vec<isize> = Vec::with_capacity(seeds.len());
    for seed in seeds {
        let mut out = seed;
        for c in &conversions {
            for r in c {
                if r.0 <= out && r.0+r.3 >= out {
                    out = out + r.2;
                    break;
                }
            }
        }
        seed_locations.push(out);
    }

    seed_locations.iter().min().unwrap().to_string()
}

pub fn y23q5q2(input: Vec<String>) -> String {
    // input range start, output range start,
    // output range start offset, range length
    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct Range(isize, isize, isize, isize);

    // start location, range length
    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
    struct CalcStep(isize, isize);

    let mut conversions: [Vec<Range>; 7] = Default::default();

    let mut curr_conversion_idx = 0;
    for line in &input[3..] {
        if line.contains("-to-") {
            curr_conversion_idx += 1;
        } else if line.is_empty() {
            continue;
        } else {
            // has 3 numbers separated by whitespace
            let rng:Vec<isize> = line.split_whitespace()
                .map(|s| s.parse::<isize>().unwrap_or(0)).collect();
            let rngg:Range = Range(rng[1], rng[0], rng[0]-rng[1], rng[2]);
            conversions[curr_conversion_idx].push(rngg);
        }
    }

    for i in 0..conversions.len() {
        conversions[i].sort();
        conversions[i].reverse();
    }

    let seeds_vec:Vec<isize> = input[0].trim_start_matches("seeds: ")
        .split_whitespace().map(|s| s.parse::<isize>().unwrap_or(0)).collect();

    let mut calc:Vec<CalcStep> = seeds_vec.chunks(2)
        .map(|chunk| CalcStep(chunk[0], chunk[1])).collect();

    for cvs in conversions {
        let calcul = calc.clone();
        calc = Vec::default();
        for step in calcul {
            // step: (input range start, range length)
            // cvs are sorted backwards!
            // cv: (input range start, output range start, output range offset, range length
            let mut curr = step.clone();
            while curr.1 > 0 {
                let mut curr_range = curr.1;
                for (i, cv) in cvs.iter().enumerate() {
                    // branch A, eg
                    // curr (79, 14) | cv (52, 48)
                    if cv.0 <= curr.0 && curr.0 < cv.0 + cv.3 {
                        curr_range = min(curr.1, cv.0 + cv.3 - curr.0);
                        calc.push(CalcStep(curr.0 + cv.2, curr_range));
                        break;
                    } else if cv.0 <= curr.0 {
                        if i > 0 {
                            if curr.0 + curr.1 >= cvs[i-1].0 {
                                curr_range = cvs[i-1].0 - curr.0;
                            }
                        }
                        calc.push(CalcStep(curr.0, curr_range));
                        break;
                    }
                    if i == cvs.len() - 1 {
                        if curr.0 + curr.1 >= cvs[i].0 {
                            curr_range = cvs[i].0 - curr.0;
                        }
                        calc.push(CalcStep(curr.0, curr_range));
                    }
                }
                curr = CalcStep(curr.0 + curr_range, curr.1 - curr_range);
            }
        }
    }

    calc.iter().min().unwrap().0.to_string()
}
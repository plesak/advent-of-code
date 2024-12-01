pub fn y23q6q1(input: Vec<String>) -> String {

    let times:Vec<usize> = input[0].trim_start_matches("Time: ")
        .split_whitespace().map(|s| s.parse::<usize>().unwrap_or(0)).collect();
    let distances:Vec<usize> = input[1].trim_start_matches("Distance: ")
        .split_whitespace().map(|s| s.parse::<usize>().unwrap_or(0)).collect();
    assert_eq!(times.len(), distances.len());

    fn quadr(t: usize, d: usize) -> (usize, usize) {
        // a = 1, b = -t, c = d
        // x = ( t +- sqrt(t^2 - 4d) ) / 2

        let discr_root = ((t.pow(2) -4*d) as f64).sqrt() as usize;

        let x1 = (t + discr_root) / 2;
        let x2 = (t - discr_root) / 2;
        (x1, x2+1)
    }

    fn calc_dist(t: usize, x: usize) -> usize {
        // t is total time allocated for race
        // x is how long you press the button for
        x * (t-x)
    }

    let mut res:Vec<usize> = Vec::default();
    for i in 0..times.len() {
        let t = times[i];
        let d = distances[i];

        let (mut x1, mut x2) = quadr(t, d);
        assert!(calc_dist(t, x1) >= d);
        assert!(calc_dist(t, x2) >= d);

        // validate upper bound
        while calc_dist(t, x1) > d {
            x1 += 1;
        }
        x1 -= 1;
        // validate lower bound
        while calc_dist(t, x2) > d {
            x2 -= 1;
        }
        x2 += 1;

        res.push(x1.saturating_sub(x2) + 1);
    }

    println!("{:?}", res);
    res.iter().product::<usize>().to_string()
}

pub fn y23q6q2(input: Vec<String>) -> String {

    let t:usize = input[0].trim_start_matches("Time:").trim()
        .split_whitespace().collect::<String>()
        .parse::<usize>().unwrap();

    let d:usize = input[1].trim_start_matches("Distance:").trim()
        .split_whitespace().collect::<String>()
        .parse::<usize>().unwrap();

    println!("time: {}, distance: {}", t, d);

    fn quadr(t: usize, d: usize) -> (usize, usize) {
        // a = 1, b = -t, c = d
        // x = ( t +- sqrt(t^2 - 4d) ) / 2

        let discr_root = ((t.pow(2) -4*d) as f64).sqrt() as usize;

        let x1 = (t + discr_root) / 2;
        let x2 = (t - discr_root) / 2;
        (x1, x2+1)
    }

    fn calc_dist(t: usize, x: usize) -> usize {
        // t is total time allocated for race
        // x is how long you press the button for
        x * (t-x)
    }

    let (mut x1, mut x2) = quadr(t, d);
    assert!(calc_dist(t, x1) >= d);
    assert!(calc_dist(t, x2) >= d);

    // validate upper bound
    while calc_dist(t, x1) > d {
        x1 += 1;
    }
    x1 -= 1;
    // validate lower bound
    while calc_dist(t, x2) > d {
        x2 -= 1;
    }
    x2 += 1;

    (x1.saturating_sub(x2) + 1).to_string()
}
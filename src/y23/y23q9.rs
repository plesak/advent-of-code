pub fn y23q9q1(input: Vec<String>) -> String {
    // ex: 10 13 16 21 30 45 -> 68
    // if all 0s, next is 0
    // else next is last plus find_next(array_of_diffs)

    fn diffs(arr:&Vec<isize>) -> Vec<isize> {
        let mut out:Vec<isize> = Vec::with_capacity(arr.len());
        for i in 0..arr.len()-1 {
            out.push(arr[i+1] - arr[i]);
        }
        out
    }

    fn find_next(arr:Vec<isize>) -> isize {
        if arr.iter().all(|&n| n == 0) { return 0; }
        find_next(diffs(&arr)) + arr.last().unwrap()
    }

    let mut sum:isize = 0;
    for ln in input {
        let arr:Vec<isize> = ln.split_whitespace()
            .map(|n| n.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        sum += find_next(arr);
    }

    sum.to_string()
}

pub fn y23q9q2(input: Vec<String>) -> String {
    // ex: 10 13 16 21 30 45 -> prev elem is 5
    // if all 0s, prev is 0
    // else prev is last plus find_next(array_of_diffs)

    fn diffs(arr:&Vec<isize>) -> Vec<isize> {
        let mut out:Vec<isize> = Vec::with_capacity(arr.len());
        for i in 0..arr.len()-1 {
            out.push(arr[i+1] - arr[i]);
        }
        out
    }

    fn find_prev(arr:Vec<isize>) -> isize {
        if arr.iter().all(|&n| n == 0) { return 0; }
        arr.first().unwrap() - find_prev(diffs(&arr))
    }

    let mut sum:isize = 0;
    for ln in input {
        let arr:Vec<isize> = ln.split_whitespace()
            .map(|n| n.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        sum += find_prev(arr);
    }

    sum.to_string()
}
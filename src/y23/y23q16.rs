use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use ndarray::Array2;

pub fn y23q16q1(input:Vec<String>) -> String {

    let nrows = input.len();
    let ncols = input[0].len();
    let flat_vec:Vec<char> = input.concat().chars().collect();
    let a =
        Array2::from_shape_vec((nrows, ncols), flat_vec).unwrap();

    #[derive(Debug)]
    // Beam(i, j, beam direction)
    // > = 2, v = 3, < = 5, ^ = 7, <v = 6, <^=35, >v<=210
    struct Beam(usize, usize, usize);
    let mut beams: VecDeque<Beam> = VecDeque::from([Beam(0, 0, 2)]);

    #[derive(Debug)]
    // Cell(content, product of all beam directions)
    struct Cell((), usize);
    // only energized cells are stored in the hashmap
    let mut cells:HashMap<usize, Cell> = HashMap::new();

    while let Some(beam) = beams.pop_front() {
        let i = beam.0;
        let j = beam.1;
        let beam_dir = beam.2;
        let c = a[[i, j]];

        let cell = cells.entry(500*i+j).or_insert(Cell((), 0));
        // if beam from this direction has already come to this cell, we would be looping
        if cell.1 > 0 && cell.1 % beam_dir == 0 {
            continue;
        }
        if cell.1 > 0 {
            cell.1 *= beam_dir;
        } else {
            cell.1 = beam_dir;
        }
        // > = 2, v = 3, < = 5, ^ = 7, <v = 6, <^=35, >v<=210
        match c {
            '.' => {
                match beam_dir {
                    2 => if j < ncols-1 { beams.push_back(Beam(i, j+1, beam_dir)) },
                    3 => if i < nrows-1 { beams.push_back(Beam(i+1, j, beam_dir)) },
                    5 => if j > 0 { beams.push_back(Beam(i, j-1, beam_dir)) },
                    7 => if i > 0 { beams.push_back(Beam(i-1, j, beam_dir)) },
                    _ => panic!("unknown beam_dir value! {}", beam_dir)
                }
            },
            '-' => {
                match beam_dir {
                    2 => if j < ncols-1 { beams.push_back(Beam(i, j+1, beam_dir)) },
                    3 | 7 => {
                        if j < ncols-1 { beams.push_back(Beam(i, j+1, 2)) };
                        if j > 0 { beams.push_back(Beam(i, j-1, 5)) };
                    },
                    5 => if j > 0 { beams.push_back(Beam(i, j-1, beam_dir)) },
                    _ => panic!("unknown beam_dir value! {}", beam_dir)
                }
            },
            '|' => {
                match beam_dir {
                    2 | 5 => {
                        if i < nrows-1 { beams.push_back(Beam(i+1, j, 3)) };
                        if i > 0 { beams.push_back(Beam(i-1, j, 7)) };
                    },
                    3 => if i < nrows-1 { beams.push_back(Beam(i+1, j, beam_dir)) },
                    7 => if i > 0 { beams.push_back(Beam(i-1, j, beam_dir)) },
                    _ => panic!("unknown beam_dir value! {}", beam_dir)
                }
            },
            // > = 2, v = 3, < = 5, ^ = 7, <v = 6, <^=35, >v<=210
            '/' => {
                match beam_dir {
                    2 => if i > 0 { beams.push_back(Beam(i-1, j, 7)) },
                    3 => if j > 0 { beams.push_back(Beam(i, j-1, 5)) },
                    5 => if i < nrows-1 { beams.push_back(Beam(i+1, j, 3)) },
                    7 => if j < ncols-1 { beams.push_back(Beam(i, j+1, 2)) },
                    _ => panic!("unknown beam_dir value! {}", beam_dir)
                }
            },
            '\\' => {
                match beam_dir {
                    2 => if i < nrows-1 { beams.push_back(Beam(i+1, j, 3)) },
                    3 => if j < ncols-1 { beams.push_back(Beam(i, j+1, 2)) },
                    5 => if i > 0 { beams.push_back(Beam(i-1, j, 7)) },
                    7 => if j > 0 { beams.push_back(Beam(i, j-1, 5)) },
                    _ => panic!("unknown beam_dir value! {}", beam_dir)
                }
            },
            _ => panic!("unknown character! {}", c)
        }
        // println!("{:?}", beams);
    }
    cells.iter().count().to_string()
}

pub fn y23q16q2(input:Vec<String>) -> String {

    // UGLY BRUTEFORCE VERSION ~5s
    let nrows = input.len();
    let ncols = input[0].len();
    let flat_vec:Vec<char> = input.concat().chars().collect();
    let a =
        Array2::from_shape_vec((nrows, ncols), flat_vec).unwrap();
    #[derive(Debug)]
    // Beam(i, j, beam direction)
    // > = 2, v = 3, < = 5, ^ = 7, <v = 6, <^=35, >v<=210
    struct Beam(usize, usize, usize);
    #[derive(Debug)]
    // Cell(content, product of all beam directions)
    struct Cell((), usize);

    fn energize(i:usize, j:usize, nrows:usize, ncols:usize, dr:usize, a:Array2<char>) -> usize {

        let mut beams: VecDeque<Beam> = VecDeque::from([Beam(i, j, dr)]);
        // only energized cells are stored in the hashmap
        let mut cells:HashMap<usize, Cell> = HashMap::new();

        while let Some(beam) = beams.pop_front() {
            let i = beam.0;
            let j = beam.1;
            let beam_dir = beam.2;
            let c = a[[i, j]];

            let cell = cells.entry(500*i+j).or_insert(Cell((), 0));
            // if beam from this direction has already come to this cell, we would be looping
            if cell.1 > 0 && cell.1 % beam_dir == 0 {
                continue;
            }
            if cell.1 > 0 {
                cell.1 *= beam_dir;
            } else {
                cell.1 = beam_dir;
            }
            // > = 2, v = 3, < = 5, ^ = 7, <v = 6, <^=35, >v<=210
            match c {
                '.' => {
                    match beam_dir {
                        2 => if j < ncols-1 { beams.push_back(Beam(i, j+1, beam_dir)) },
                        3 => if i < nrows-1 { beams.push_back(Beam(i+1, j, beam_dir)) },
                        5 => if j > 0 { beams.push_back(Beam(i, j-1, beam_dir)) },
                        7 => if i > 0 { beams.push_back(Beam(i-1, j, beam_dir)) },
                        _ => panic!("unknown beam_dir value! {}", beam_dir)
                    }
                },
                '-' => {
                    match beam_dir {
                        2 => if j < ncols-1 { beams.push_back(Beam(i, j+1, beam_dir)) },
                        3 | 7 => {
                            if j < ncols-1 { beams.push_back(Beam(i, j+1, 2)) };
                            if j > 0 { beams.push_back(Beam(i, j-1, 5)) };
                        },
                        5 => if j > 0 { beams.push_back(Beam(i, j-1, beam_dir)) },
                        _ => panic!("unknown beam_dir value! {}", beam_dir)
                    }
                },
                '|' => {
                    match beam_dir {
                        2 | 5 => {
                            if i < nrows-1 { beams.push_back(Beam(i+1, j, 3)) };
                            if i > 0 { beams.push_back(Beam(i-1, j, 7)) };
                        },
                        3 => if i < nrows-1 { beams.push_back(Beam(i+1, j, beam_dir)) },
                        7 => if i > 0 { beams.push_back(Beam(i-1, j, beam_dir)) },
                        _ => panic!("unknown beam_dir value! {}", beam_dir)
                    }
                },
                // > = 2, v = 3, < = 5, ^ = 7, <v = 6, <^=35, >v<=210
                '/' => {
                    match beam_dir {
                        2 => if i > 0 { beams.push_back(Beam(i-1, j, 7)) },
                        3 => if j > 0 { beams.push_back(Beam(i, j-1, 5)) },
                        5 => if i < nrows-1 { beams.push_back(Beam(i+1, j, 3)) },
                        7 => if j < ncols-1 { beams.push_back(Beam(i, j+1, 2)) },
                        _ => panic!("unknown beam_dir value! {}", beam_dir)
                    }
                },
                '\\' => {
                    match beam_dir {
                        2 => if i < nrows-1 { beams.push_back(Beam(i+1, j, 3)) },
                        3 => if j < ncols-1 { beams.push_back(Beam(i, j+1, 2)) },
                        5 => if i > 0 { beams.push_back(Beam(i-1, j, 7)) },
                        7 => if j > 0 { beams.push_back(Beam(i, j-1, 5)) },
                        _ => panic!("unknown beam_dir value! {}", beam_dir)
                    }
                },
                _ => panic!("unknown character! {}", c)
            }
            // println!("{:?}", beams);
        }
        cells.iter().count()
    }

    let mut max_energy = 0;
    // top row
    for j in 0..ncols {
        max_energy = max(max_energy, energize(0, j, nrows, ncols, 3, a.clone()));
        max_energy = max(max_energy, energize(nrows-1, j, nrows, ncols, 7, a.clone()));
    }
    for i in 0..nrows {
        max_energy = max(max_energy, energize(i, 0, nrows, ncols, 2, a.clone()));
        max_energy = max(max_energy, energize(i, ncols-1, nrows, ncols, 5, a.clone()));
    }

    max_energy.to_string()
}
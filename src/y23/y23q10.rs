pub fn y23q10q1(input: Vec<String>) -> String {

    #[derive(Debug)]
    struct Coords(usize, usize);
    let mut coords:Coords = Coords(0, 0);

    let n_rows:usize = input.len();
    let n_cols:usize = input[0].len();

    let mut map:Vec<Vec<char>> = Vec::with_capacity(input.len());
    for (i, ln) in input.iter().enumerate() {
        let mut row:Vec<char> = Vec::with_capacity(ln.len());
        for (j, ch) in ln.chars().enumerate() {
            if ch == 'S' { coords = Coords(i, j); }
            row.push(ch);
        }
        map.push(row);
    }

    let mut last_dir = "unknown";
    let mut curr_c:char;
    // find any pipe out of S (guaranteed by problem to be on path)
    let mut found_path = false;
    if coords.0 != 0 {
        curr_c = map[coords.0-1][coords.1];
        if curr_c == '7' || curr_c == '|' || curr_c == 'F' {
            last_dir = "up";
            coords.0 -= 1;
            found_path = true;
        }
    }
    if !found_path && coords.1 != n_cols {
        curr_c = map[coords.0][coords.1+1];
        if curr_c == 'J' || curr_c == '-' || curr_c == '7' {
            last_dir = "right";
            coords.1 += 1;
            found_path = true;
        }
    }
    if !found_path && coords.0 != n_rows {
        curr_c = map[coords.0+1][coords.1];
        if curr_c == 'J' || curr_c == '|' || curr_c == 'L' {
            last_dir = "down";
            coords.0 += 1;
            found_path = true;
        }
    }
    if !found_path && coords.1 != 0 {
        curr_c = map[coords.0][coords.1-1];
        if curr_c == 'F' || curr_c == '-' || curr_c == 'L' {
            last_dir = "left";
            coords.1 -= 1;
            found_path = true;
        }
    }
    if !found_path { panic!("Coudln't find a starting path!!"); }

    let mut steps = 1;
    curr_c = map[coords.0][coords.1];
    while curr_c != 'S' {
        // println!("pipe {} at {:?}", curr_c, coords);
        match curr_c {
            '|' => {
                match last_dir {
                    "up" => coords.0 -= 1,
                    "down" => coords.0 += 1,
                    _ => panic!("Wrong direction at {:?}!", coords)
                }
            },
            'F' => {
                match last_dir {
                    "up" => { coords.1 += 1; last_dir = "right" },
                    "left" => { coords.0 += 1; last_dir = "down" },
                    _ => panic!("Wrong direction at {:?}!", coords)
                }
            }
            '-' => {
                match last_dir {
                    "left" => coords.1 -= 1,
                    "right" => coords.1 += 1,
                    _ => panic!("Wrong direction at {:?}!", coords)
                }
            }
            'L' => {
                match last_dir {
                    "down" => { coords.1 += 1; last_dir = "right" },
                    "left" => { coords.0 -= 1; last_dir = "up" },
                    _ => panic!("Wrong direction at {:?}!", coords)
                }
            }
            'J' => {
                match last_dir {
                    "down" => { coords.1 -= 1; last_dir = "left" },
                    "right" => { coords.0 -= 1; last_dir = "up" },
                    _ => panic!("Wrong direction at {:?}!", coords)
                }
            }
            '7' => {
                match last_dir {
                    "up" => { coords.1 -= 1; last_dir = "left" },
                    "right" => { coords.0 += 1; last_dir = "down" },
                    _ => panic!("Wrong direction at {:?}!", coords)
                }
            }
            _ => panic!("Unknown pipe {} at {:?}!", curr_c, coords)
        }
        steps += 1;
        curr_c = map[coords.0][coords.1];
    }

    (steps/2).to_string()

}

pub fn y23q10q2(input: Vec<String>) -> String {

    #[derive(Debug)]
    struct Coords(usize, usize);
    let mut coords:Coords = Coords(0, 0);

    let n_rows:usize = input.len();
    let n_cols:usize = input[0].len();

    let mut map:Vec<Vec<char>> = Vec::with_capacity(n_rows);
    for (i, ln) in input.iter().enumerate() {
        let mut row:Vec<char> = Vec::with_capacity(n_cols);
        for (j, ch) in ln.chars().enumerate() {
            if ch == 'S' { coords = Coords(i, j); }
            row.push(ch);
        }
        map.push(row);
    }

    // find both pipes out of S
    // 2=up 3=right 5=down 7=left
    let mut start_dirs:Vec<i32> = Vec::with_capacity(2);

    if coords.0 != 0 {
        let next_c = map[coords.0-1][coords.1];
        if next_c == '7' || next_c == '|' || next_c == 'F' {
            start_dirs.push(2);
        }
    }
    if coords.1 != n_cols {
        let next_c = map[coords.0][coords.1+1];
        if next_c == 'J' || next_c == '-' || next_c == '7' {
            start_dirs.push(3);
        }
    }
    if coords.0 != n_rows {
        let next_c = map[coords.0+1][coords.1];
        if next_c == 'J' || next_c == '|' || next_c == 'L' {
            start_dirs.push(5);
        }
    }
    if coords.1 != 0 {
        let next_c = map[coords.0][coords.1-1];
        if next_c == 'F' || next_c == '-' || next_c == 'L' {
            start_dirs.push(7);
        }
    }
    if !start_dirs.len() == 2 { panic!("Startup issue {:?}!", start_dirs); }

    let mut last_dir:&str;
    match start_dirs[0]+start_dirs[1] {
        5 => {
            // 2 + 3 -- up + right -- L
            map[coords.0][coords.1] = 'U';
            last_dir = "up";
            coords.0 -= 1;
        },
        7 => {
            // 2 + 5 -- up + down -- |
            map[coords.0][coords.1] = 'T';
            last_dir = "up";
            coords.0 -= 1;
        },
        9 => {
            // 2 + 7 -- up + left -- J
            map[coords.0][coords.1] = 'U';
            last_dir = "up";
            coords.0 -= 1;
        },
        8 => {
            // 3 + 5 -- right + down -- F
            map[coords.0][coords.1] = 'D';
            last_dir = "right";
            coords.1 += 1;
        },
        10 => {
            // 3 + 7 -- right + left -- -
            map[coords.0][coords.1] = 'P';
            last_dir = "right";
            coords.1 += 1;
        },
        12 => {
            // 5 + 7 -- down + left -- 7
            map[coords.0][coords.1] = 'D';
            last_dir = "down";
            coords.0 += 1;
        },
        _ => panic!("Starting weird {:?}!", start_dirs)
    };

    let mut curr_c = map[coords.0][coords.1];
    while !['P', 'T', 'U', 'D'].contains(&curr_c) {
        // println!("pipe {} at {:?}", curr_c, coords);
        match curr_c {
            '|' => {
                map[coords.0][coords.1] = 'T';
                match last_dir {
                    "up" => coords.0 -= 1,
                    "down" => coords.0 += 1,
                    _ => panic!("Wrong direction at {:?}!", coords)
                }
            },
            'F' => {
                map[coords.0][coords.1] = 'D';
                match last_dir {
                    "up" => { coords.1 += 1; last_dir = "right" },
                    "left" => { coords.0 += 1; last_dir = "down" },
                    _ => panic!("Wrong direction at {:?}!", coords)
                }
            }
            '-' => {
                map[coords.0][coords.1] = 'P';
                match last_dir {
                    "left" => coords.1 -= 1,
                    "right" => coords.1 += 1,
                    _ => panic!("Wrong direction at {:?}!", coords)
                }
            }
            'L' => {
                map[coords.0][coords.1] = 'U';
                match last_dir {
                    "down" => { coords.1 += 1; last_dir = "right" },
                    "left" => { coords.0 -= 1; last_dir = "up" },
                    _ => panic!("Wrong direction at {:?}!", coords)
                }
            }
            'J' => {
                map[coords.0][coords.1] = 'U';
                match last_dir {
                    "down" => { coords.1 -= 1; last_dir = "left" },
                    "right" => { coords.0 -= 1; last_dir = "up" },
                    _ => panic!("Wrong direction at {:?}!", coords)
                }
            }
            '7' => {
                map[coords.0][coords.1] = 'D';
                match last_dir {
                    "up" => { coords.1 -= 1; last_dir = "left" },
                    "right" => { coords.0 += 1; last_dir = "down" },
                    _ => panic!("Wrong direction at {:?}!", coords)
                }
            }
            _ => panic!("Unknown pipe {} at {:?}!", curr_c, coords)
        }
        curr_c = map[coords.0][coords.1];
    }

    // println!("{:?}", map);

    let mut count_inside = 0;
    for ln in map.into_iter() {
        let mut inside = false;
        let mut horizontal_start = 'X';
        for c in ln {
            match c {
                'T' => { inside = !inside; },
                'U' | 'D' => {
                    if horizontal_start == 'X' {
                        horizontal_start = c;
                    } else if c == horizontal_start {
                        // L--J or F--7
                        horizontal_start = 'X';
                    } else if c != horizontal_start {
                        // L--7 or F--J
                        inside = !inside;
                        horizontal_start = 'X';
                    }
                },
                'P' => {},
                _ => {
                    if inside { count_inside += 1; }
                }
            };
        }
        // println!("cnt in after row {:?}", count_inside);
    }

    count_inside.to_string()
}
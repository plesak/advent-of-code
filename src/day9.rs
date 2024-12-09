use num::Integer;
use std::collections::VecDeque;
use std::fmt::Display;

pub fn part1(inp: &str) -> impl Display {
    // 2333133121414131402
    // means 00...111...2...333.44.5555.6666.777.888899
    let ln = inp.len() as i64;
    let mut curr_back_id = (ln - 1) / 2;

    // verify if last digit is space or file
    let mut back_counter = 0;
    let rev_map = inp;
    let rev_map = rev_map.chars().rev().collect::<String>();
    let mut rev_map_iter = rev_map.chars();
    if inp.len().is_even() { rev_map_iter.next(); }

    // main sum
    let mut space_counter = 0;
    let mut curr_idx = 0;
    let mut leftover_file_size = 0;

    for (i, c) in inp.trim().chars().enumerate() {
        if ln - back_counter - 1 < i as i64 {
            // println!("we hit each other!! at {}", i);
            break;
        } else if ln - back_counter - 1 == i as i64 {
            // println!("we hit each other!! at {}", i);
            if leftover_file_size > 0 {
                for x in curr_idx..curr_idx + leftover_file_size {
                    // println!("adding {}*{}", x, curr_back_id);
                    space_counter += x * curr_back_id;
                }
            }
            break;
        }
        let n = c.to_digit(10).unwrap() as i64;
        if i.is_even() {
            // from the front, file ID is just i/2
            for x in curr_idx..curr_idx + n {
                // println!("adding {}*{}", x, (i as i64)/2);
                space_counter += x * (i as i64) / 2;
            }
            curr_idx += n;
        } else {
            let mut empty_space = n;
            while empty_space > 0 {
                if ln - back_counter - 1 <= i as i64 {
                    // println!("we hit each other!! at {}", i);
                    break;
                }
                let mut file_size;
                if leftover_file_size > 0 {
                    file_size = leftover_file_size;
                } else {
                    // otherwise we take first available file from the back
                    file_size = rev_map_iter.next().unwrap().to_digit(10).unwrap() as i64;
                }
                let file_id = curr_back_id;
                if file_size > empty_space {
                    for x in curr_idx..curr_idx + empty_space {
                        // println!("adding {}*{}", x, file_id);
                        space_counter += x * file_id;
                    }
                    curr_idx += empty_space;
                    leftover_file_size = file_size - empty_space;
                    empty_space = 0;
                } else {
                    for x in curr_idx..curr_idx + file_size {
                        // println!("adding {}*{}", x, file_id);
                        space_counter += x * file_id;
                    }
                    curr_idx += file_size;
                    back_counter += 2; // +1 for the next empty space
                    leftover_file_size = 0;
                    empty_space -= file_size;
                    curr_back_id -= 1;
                    rev_map_iter.next();
                }
            }
        }
    }
    space_counter
}

pub fn part2(inp: &str) -> impl Display {
    // 2333133121414131402
    // means 00...111...2...333.44.5555.6666.777.888899
    // compressed 00992111777.44.333....5555.6666.....8888..
    // or 12345
    // means 0..111....22222
    // compressed 0..111....22222
    let ln = inp.len() as i64;
    let mut curr_back_id = (ln - 1) / 2;
    let mut curr_back_ix = ln - 1;
    if ln.is_even() { curr_back_ix -= 1; }

    let v_nums = inp.trim().chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();

    let mut v = inp.trim().chars().enumerate()
        .map(|(i, c)| (
            c.to_digit(10).unwrap() as i64,
            i.is_odd(),
            if i.is_even() { (i as i64) / 2 } else { -1 }
        )).collect::<VecDeque<(i64, bool, i64)>>();

    // println!("mutv is {:?}", v);

    for ix in (0..=curr_back_ix).rev() {
        if ix.is_even() {
            let file_size: i64 = v_nums[ix as usize];
            // println!("trying to move file {} from ix {}", curr_back_id, ix);

            let mut found_loc = 99999999;
            let mut found_own_ix = 99999999;
            for (i, &loc) in v.iter().enumerate() {
                if loc.2 == curr_back_id {
                    found_own_ix = i;
                    break;
                } // we got to its original spot
                if loc.1 == true { // this is a free space we can try to occupy
                    if loc.0 >= file_size && found_loc == 99999999 {
                        // println!("gonna populate the file into {:?}", loc);
                        found_loc = i;
                    }
                }
            }
            if found_loc != 99999999 {
                // replace original with empty space
                v.remove(found_own_ix);
                v.insert(found_own_ix, (file_size, true, -1));
                // move to new space
                let og_space = v[found_loc].0;
                v.remove(found_loc);
                v.insert(found_loc, (file_size, false, curr_back_id));
                if og_space - file_size > 0 {
                    v.insert(found_loc + 1, (og_space - file_size, true, -1));
                }
                // println!("current mutv {:?}", v);
            }

            curr_back_ix -= 1;
            curr_back_id -= 1;
        } else {
            curr_back_ix -= 1;
        }
    }

    let mut final_sum = 0;
    let mut ix_counter = 0;
    for &elem in v.iter() {
        if elem.1 == false {
            for x in ix_counter..ix_counter + elem.0 {
                final_sum += x * elem.2;
                ix_counter += 1;
            }
        } else {
            ix_counter += elem.0;
        }
    }

    final_sum
}
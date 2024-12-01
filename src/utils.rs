use ndarray::Array2;
use num::abs;

pub fn inp_to_array2(input:Vec<String>) -> Array2<char> {
    let nrows = input.len();
    let ncols = input[0].len();
    let flat_vec:Vec<char> = input.concat().chars().collect();
    Array2::from_shape_vec((nrows, ncols), flat_vec).unwrap()
}

// pub fn flood_from_coords(coords:Vec<(i64, i64)>, incl_border:bool) -> i64 {
//     // calculates the number of grid points flooded from a list of vertices
//     // list of vertices must be in order (clockwise or counterclockwise)
//     // may or may not include border vertices
//     if incl_border {
//         coords[0].0
//     } else {
//         coords[0].1
//     }
// }

pub fn flood_from_paths(paths:Vec<(i64, char)>, incl_border:bool) -> i64 {
    // calculates the number of grid points flooded from a list of distance+dirs
    // dir: R, D, L, U
    // list of vertices must be in order (clockwise or counterclockwise)
    // may or may not include border vertices

    let mut shoelace_sum:i64 = 0;
    let mut curr_coord:(i64, i64) = (0, 0);
    let mut total_path = 0;
    for path in paths {
        let dist = path.0;
        total_path += dist;
        let new_coord:(i64, i64) = match path.1 {
            'R' => (curr_coord.0, curr_coord.1 + dist),
            'D' => (curr_coord.0 + dist, curr_coord.1),
            'L' => (curr_coord.0, curr_coord.1 - dist),
            'U' => (curr_coord.0 - dist, curr_coord.1),
            _ => panic!("unknown direction {}", path.1)
        };
        shoelace_sum += curr_coord.0*new_coord.1 - curr_coord.1*new_coord.0;
        curr_coord = new_coord;
    }

    let mut points = (abs(shoelace_sum) - total_path)/2 + 1;
    if incl_border {
        points += total_path;
    }
    points
}



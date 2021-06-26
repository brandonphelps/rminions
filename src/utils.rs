#![allow(dead_code)]

/// helper functions that aren't specific the app.

// todo: how to make T only be valid for ints?
// todo: do all data types need be same type?
// pub fn manhat_distance<T>(x1: T, y1: T, x2: T, y2: T) -> T
// where
//     T: std::ops::Sub<Output = T> + std::cmp::PartialOrd
pub fn manhat_distance(x1: u32, y1: u32, x2: u32, y2: u32) -> u32 {
    let x_dist: i32 = (x1 as i32 - x2 as i32).into();
    let y_dist: i32 = (y1 as i32 - y2 as i32).into();
    x_dist.abs() as u32 + y_dist.abs() as u32
}

// whats that dudes name?
pub fn uclid_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let x_dist = (x2 - x1).powf(2.0);
    let y_dist = (y2 - y1).powf(2.0);
    return (x_dist + y_dist).sqrt();
}

pub struct Path {
    // should use position?
    pub path_points: Vec<(u32, u32)>,
}

impl Path {
    pub fn new() -> Path {
        Path {
            path_points: Vec::new(),
        }
    }

    pub fn add_return_path(&mut self) {
        let mut p = self.path_points.clone();
        p.pop();
        let mut reverse = p.into_iter().rev().collect();
        self.path_points.append(&mut reverse);
    }
}

pub fn generate_path_step(start_pos: (f32, f32), end_pos: (f32, f32), delta: f32) -> Path {
    let mut r_path = Path::new();

    let mut current_pos = start_pos;
    while current_pos != end_pos {
        let mut next_x = current_pos.0;
        let mut next_y = current_pos.1;
        if current_pos.0 < end_pos.0 {
            next_x = current_pos.0 + delta;
        } else if current_pos.0 > end_pos.0 {
            next_x = current_pos.0 - delta;
        } else {
            // movement is restricted to 1 tile at a time.
            // thus no diagional movement on this non best-agon grided layout
            // todo: change grid layout to best-agons
            if current_pos.1 < end_pos.1 {
                next_y = current_pos.1 + delta;
            } else if current_pos.1 > end_pos.1 {
                next_y = current_pos.1 - delta;
            }
        }
        current_pos = (next_x, next_y);
        r_path
            .path_points
            .push((current_pos.0 as u32, current_pos.1 as u32));
    }
    return r_path;
}

/// naive direct path handling, no detection of things in the way in the slight est.
pub fn generate_path(start_pos: (u32, u32), end_pos: (u32, u32)) -> Path {
    let mut r_path = Path::new();

    let mut current_pos = start_pos;
    while current_pos != end_pos {
        let mut next_x = current_pos.0;
        let mut next_y = current_pos.1;
        if current_pos.0 < end_pos.0 {
            next_x = current_pos.0 + 1;
        } else if current_pos.0 > end_pos.0 {
            next_x = current_pos.0 - 1;
        } else {
            // movement is restricted to 1 tile at a time.
            // thus no diagional movement on this non best-agon grided layout
            // todo: change grid layout to best-agons
            if current_pos.1 < end_pos.1 {
                next_y = current_pos.1 + 1;
            } else if current_pos.1 > end_pos.1 {
                next_y = current_pos.1 - 1;
            }
        }
        current_pos = (next_x, next_y);
        r_path.path_points.push(current_pos);
    }
    return r_path;
}

/// Returns a list of internals where each tuple contains a start and end point.
/// space is the amount between each internal.
/// count, number of intervals to return.
/// first internal will start at [0, space].
// length of vec == count
pub fn spaced_internals(space: u32, count: u32) -> Vec<(u32, u32)> {
    let mut res = Vec::new();
    for p in 0..count {
        res.push((p * space, (p * space) + space));
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_start_end_same() {
        let start_p = (0, 0);
        let end_p = (0, 0);
        let result_p = generate_path(start_p, end_p);
        assert_eq!(result_p.path_points.len(), 0);
    }

    #[test]
    fn test_interval_calc() {
        let expected_res = vec![(0, 10), (10, 20), (20, 30)];
        assert_eq!(spaced_internals(10, 3), expected_res);
    }

    #[test]
    fn test_path_step_distance_one() {
        let result_p = generate_path((0, 0), (10, 0));
        for chunk in result_p.path_points.chunks(2) {
            assert_eq!(
                manhat_distance(chunk[0].0, chunk[0].1, chunk[1].0, chunk[1].1),
                1
            );
        }
        assert_eq!(result_p.path_points.len(), 10);
    }

    #[test]
    fn test_path_step_distance_one_angles() {
        let result_p = generate_path((0, 0), (10, 10));
        for chunk in result_p.path_points.chunks(2) {
            assert_eq!(
                manhat_distance(chunk[0].0, chunk[0].1, chunk[1].0, chunk[1].1),
                1
            );
        }
    }

    #[test]
    fn test_path_no_repeat() {
        let mut result_p = generate_path((0, 0), (10, 3));
        let mut p = result_p.path_points.pop();
        while p.is_some() {
            for p2 in result_p.path_points.iter() {
                let k = p.unwrap();
                assert_eq!(k.0 == p2.0 && k.1 == p2.1, false);
                // todo: why is this not okay?
                // can't do this "expected tuple, found a &(u32, u32)???
                //assert_eq!(p.unwrap(), p2);
            }
            p = result_p.path_points.pop();
        }
    }

    #[test]
    /// check against that when flipping the path no neighrs are next to each other
    /// regular repeats will occur.
    fn test_path_reverse_no_neighbor_repeat() {
        let mut result_p = generate_path((0, 0), (10, 3));
        result_p.add_return_path();
        for chunk in result_p.path_points.windows(2) {
            if chunk.len() == 2 {
                assert_ne!(chunk[0], chunk[1]);
            }
        }
    }

    #[test]
    fn manhat_dist() {
        assert_eq!(2, manhat_distance(0, 0, 1, 1));
    }
}

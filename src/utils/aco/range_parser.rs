use std::ops::Range;

pub fn parse_range(left: String, right: String) -> Option<Range<f64>> {
    let left_x = left.parse::<f64>();
    let right_x = right.parse::<f64>();

    match (left_x, right_x) {
        (Ok(coord_left_x), Ok(coord_right_x)) => {
            Some(coord_left_x..coord_right_x)
        },
        _ => None
    }
}
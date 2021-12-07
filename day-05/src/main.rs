use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input"));

    let lines = get_input::get_input_as_strings(INPUT);
    println!("Part 1: {}", calculate_segments_overlap(&lines, true));
    println!("Part 2: {}", calculate_segments_overlap(&lines, false));
}

fn calculate_segments_overlap(segments: &[String], ignore_diagonal: bool) -> usize {
    let mut points: HashSet<(i32, i32)> = HashSet::new();
    let mut overlapping: HashSet<(i32, i32)> = HashSet::new();
    for segment in segments {
        let coords: Vec<Vec<i32>> = segment
            .split(" -> ")
            .map(|c| c.split(',').map(|x| x.parse::<i32>().unwrap()).collect())
            .collect();
        let segment_points = map_segment_to_points(
            (coords[0][0], coords[0][1]),
            (coords[1][0], coords[1][1]),
            ignore_diagonal,
        );
        if segment_points.is_some() {
            for point in segment_points.unwrap() {
                let exists = points.insert(point);
                if !exists && !overlapping.contains(&point) {
                    overlapping.insert(point);
                }
            }
        }
    }

    overlapping.len()
}

fn map_segment_to_points(
    point1: (i32, i32),
    point2: (i32, i32),
    ignore_diagonal: bool,
) -> Option<Vec<(i32, i32)>> {
    let mut points: Vec<(i32, i32)> = Vec::new();

    let (p1x, p1y) = point1;
    let (p2x, p2y) = point2;
    let dx = p2x - p1x;
    let dy = p2y - p1y;

    if ignore_diagonal && dx != 0 && dy != 0 {
        return None;
    }

    let point_amt = if dx == 0 { dy.abs() } else { dx.abs() };

    for i in 0..point_amt + 1 {
        let pt = (p1x + (i * dx / point_amt), p1y + (i * dy / point_amt));
        points.push(pt);
    }

    Some(points)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/test"));
    #[test]
    fn test_calculate_straight_segments_overlap_ignore_diagonal() {
        let lines = get_input::get_input_as_strings(INPUT);
        let overlap = calculate_segments_overlap(&lines, true);
        assert_eq!(5, overlap);
    }
    #[test]
    fn test_calculate_straight_segments_overlap() {
        let lines = get_input::get_input_as_strings(INPUT);
        let overlap = calculate_segments_overlap(&lines, false);
        assert_eq!(12, overlap);
    }
    #[test]
    fn test_map_segment_to_points_ignore_diagonal() {
        assert_eq!(
            4,
            map_segment_to_points((1, 6), (1, 3), true).unwrap().len()
        );
        assert!(map_segment_to_points((2, 6), (1, 3), true).is_none());
    }
    #[test]
    fn test_map_segment_to_points() {
        assert_eq!(
            4,
            map_segment_to_points((1, 6), (1, 3), false).unwrap().len()
        );
        assert_eq!(
            4,
            map_segment_to_points((4, 8), (1, 5), false).unwrap().len()
        );
    }
}

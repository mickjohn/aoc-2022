use super::solution::*;

fn get_highest_y_point(points: &Vec<Point>) -> usize {
  let mut all_points: Vec<usize> = points.iter().map(|p| p.y).collect();
  all_points.sort_by(|a, b| b.cmp(a));
  all_points[0]
}

fn get_lowest_x_point(points: &Vec<Point>) -> usize {
  let mut all_points: Vec<usize> = points.iter().map(|p| p.x).collect();
  all_points.sort();
  all_points[0]
}

fn get_highest_x_point(points: &Vec<Point>) -> usize {
  let mut all_points: Vec<usize> = points.iter().map(|p| p.x).collect();
  all_points.sort_by(|a, b| b.cmp(a));
  all_points[0]
}

pub fn print(map: &Cave) {
    let all_points: Vec<Point> = map.keys().copied().collect();
    let x_start= get_lowest_x_point(&all_points);
    let x_end = get_highest_x_point(&all_points);
    let y_end = get_highest_y_point(&all_points)+4;

    for y in 0 ..= y_end {
        for x in x_start ..= x_end {
            match map.get(&Point::new(x, y)) {
                None => print!("."),
                Some(Marker::Rock) => print!("#"),
                Some(Marker::Sand) => print!("O"),
            }
        }
        println!();
    }
}

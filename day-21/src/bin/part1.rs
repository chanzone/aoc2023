use std::fmt::Debug;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    is_rock: bool,
    is_reached: bool,
}

fn main() {
    // When done, run `cargo run --bin <folder name> to get the output`

    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn extract(input: &str) -> Vec<Point> {
    let garden: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut points: Vec<Point> = vec![];

    for y in 0..garden.len() {
        for x in 0..garden[0].len() {
            let point: Point = match garden[y][x] {
                'S' => Point { x, y, is_rock: false, is_reached: true },
                '#' => Point { x, y, is_rock: true, is_reached: false },
                _ => Point { x, y, is_rock: false, is_reached: false },
            };

            points.push(point);
        }
    }

    points
}

fn process(input: &str) -> String {
    // let total_steps: usize = 6;  // For test dataset
    let total_steps: usize = 64;    // For actual dataset
    let mut points = extract(input);

    for _ in 0..total_steps {
        let reached_points: Vec<usize> = points
            .iter()
            .enumerate()
            .filter(|(_, p)| p.is_reached)
            .map(|(i, _)| i)
            .collect();

        for index in reached_points {
            let point = &mut points[index];
            let neighbors = [
                (point.x - 1, point.y),
                (point.x + 1, point.y),
                (point.x, point.y - 1),
                (point.x, point.y + 1),
            ];

            point.is_reached = false;

            for (nx, ny) in neighbors.iter() {
                if let Some(reached_index) = points.iter().position(|p| !p.is_rock && p.x == *nx && p.y == *ny) {
                    let reached_point = &mut points[reached_index];
                    reached_point.is_reached = true;
                }
            }
        }
    }

    points.iter().filter(|p| p.is_reached).count().to_string()   // Replace with the actual code here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // For test, run `cargo test --bin <folder name> to verify the test output`

        let input = include_str!("./input-test.txt");
        let result = process(input);
        assert_eq!(result, "16".to_string()); // Replace with the expected test output here
    }
}
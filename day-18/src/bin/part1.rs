#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct DigPlan {
    dir: Direction,
    steps: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
    dig: bool,
}

fn main() {
    // When done, run `cargo run --bin <folder name> to get the output`

    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn extract(input: &str) -> Vec<DigPlan> {
    input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let dir = match parts.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let steps = parts.next().unwrap().parse::<usize>().unwrap();
        let _ = parts.next().unwrap().to_string();
        DigPlan { dir, steps }
    }).collect()
}

fn shoelace_formula_area(positions: &Vec<Position>) -> i64 {
    ((1..positions.len() - 1)
        .map(|i| positions[i].x * (positions[i + 1].y - positions[i - 1].y))
        .map(|x| x as i64)
        .sum::<i64>()
        / 2)
    .abs()
}

fn exterior_point_count(positions: &[Position]) -> i64 {
    positions
        .windows(2)
        .map(|window| (window[0].x - window[1].x).abs() + (window[0].y - window[1].y).abs())
        .map(|x| x as i64)
        .sum()
}

fn process(input: &str) -> String {
    let dig_plans: Vec<DigPlan> = extract(input);
    let mut floor: Vec<Position> = vec![];

    let mut x: isize = 0;
    let mut y: isize = 0;
    for plan in dig_plans.iter() {
        for _ in 0..plan.steps {
            match plan.dir {
                Direction::Up => y += 1,
                Direction::Down => y -= 1,
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
            }
            floor.push(Position { x, y , dig: true});
        }
    }

    ((shoelace_formula_area(&floor) - exterior_point_count(&floor) / 2 + 1) + exterior_point_count(&floor)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // For test, run `cargo test --bin <folder name> to verify the test output`

        let input = include_str!("./input-test.txt");
        let result = process(input);
        assert_eq!(result, "62".to_string()); // Replace with the expected test output here
    }
}
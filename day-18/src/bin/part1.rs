// NOTE: NOT WORKING.
type Length = Vec<isize>;

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

#[derive(Debug, PartialEq)]
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

fn get_width(dig_plans: &Vec<DigPlan>) -> Length {
    let mut moved: isize = 0;
    let mut max: isize = 0;
    let mut min: isize = 0;
    for dig_plan in dig_plans {
        match dig_plan.dir {
            Direction::Right => moved += dig_plan.steps as isize,
            Direction::Left => moved -= dig_plan.steps as isize,
            _ => (),
        }

        if max < moved {
            max = moved;
        }

        if min > moved {
            min = moved;
        }
    }

    let mut current: isize = min;
    let mut width: Vec<isize> = vec![];
    while current <= max {
        width.push(current);
        current += 1;
    }

    width
}

fn get_height(dig_plans: &Vec<DigPlan>) -> Length {
    let mut moved: isize = 0;
    let mut max: isize = 0;
    let mut min: isize = 0;
    for dig_plan in dig_plans {
        match dig_plan.dir {
            Direction::Down => moved -= dig_plan.steps as isize,
            Direction::Up => moved += dig_plan.steps as isize,
            _ => (),
        }

        if max < moved {
            max = moved;
        }

        if min > moved {
            min = moved;
        }
    }

    let mut current: isize = max;
    let mut height: Vec<isize> = vec![];
    while current >= min {
        height.push(current);
        current -= 1;
    }

    height
}

fn process(input: &str) -> String {
    let dig_plans: Vec<DigPlan> = extract(input);
    let width = get_width(&dig_plans);
    let height = get_height(&dig_plans);
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
            // dbg!(x, y);
            floor.push(Position { x, y , dig: true});
        }
    }

    let mut count: usize = 0;
    for y in height.clone() {
        let mut is_within_boundary: bool = false;
        for x in width.clone() {
            if floor.contains(&Position { x, y, dig: true}) && !is_within_boundary {
                is_within_boundary = true;
                count += 1;
            } else if !floor.contains(&Position { x, y, dig: true}) && is_within_boundary {
                count += 1;
            } else if floor.contains(&Position { x, y, dig: true}) && is_within_boundary && !floor.contains(&Position { x: x - 1, y, dig: true}) {
                is_within_boundary = false;
                count += 1;
            } else if floor.contains(&Position { x, y, dig: true}) {
                count += 1;
            }
        }
    }

    count.to_string()   // Replace with the actual code here
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
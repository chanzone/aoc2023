// NOTE: NOT WORKING.
#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Position {
    x: usize,
    y: usize,
}

fn navigate(heat_maps: &Vec<Vec<char>>, heats: &mut Vec<usize>, direction: Direction, position: Position, mut straight_counter: usize, mut heat_counter: usize) {
    if position.x == heat_maps[0].len() - 1 && position.y == heat_maps.len() - 1 {
        heats.push(heat_counter);
        return;
    }

    let mut next_directions: Vec<Direction> = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    match direction {
        Direction::Up => next_directions.remove(1),
        Direction::Down => next_directions.remove(0),
        Direction::Left => next_directions.remove(3),
        Direction::Right => next_directions.remove(2),
    };

    if straight_counter >= 3 {
        straight_counter = 0;
        
        // Remove the direection from next_directions
        match direction {
            Direction::Up => next_directions.remove(0),
            Direction::Down => next_directions.remove(0),
            Direction::Left => next_directions.remove(2),
            Direction::Right => next_directions.remove(2),
        };
    }

    straight_counter += 1;
    heat_counter += 1;

    for next_direction in next_directions {
        match next_direction {
            Direction::Up => {
                if position.y > 0 {
                    let next_position = Position { x: position.x, y: position.y - 1 };
                    navigate(heat_maps, heats, next_direction, next_position, straight_counter.clone(), heat_counter.clone());
                }
            },
            Direction::Down => {
                if position.y < heat_maps.len() - 1 {
                    let next_position = Position { x: position.x, y: position.y + 1 };
                    navigate(heat_maps, heats, next_direction, next_position, straight_counter.clone(), heat_counter.clone());
                }
            },
            Direction::Left => {
                if position.x > 0 {
                    let next_position = Position { x: position.x - 1, y: position.y };
                    navigate(heat_maps, heats, next_direction, next_position, straight_counter.clone(), heat_counter.clone());
                }
            },
            Direction::Right => {
                if position.x < heat_maps[0].len() - 1 {
                    let next_position = Position { x: position.x + 1, y: position.y };
                    navigate(heat_maps, heats, next_direction, next_position, straight_counter.clone(), heat_counter.clone());
                }
            }
        }
    }

}

fn main() {
    // When done, run `cargo run --bin <folder name> to get the output`

    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let heat_maps: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut heats: Vec<usize> = vec![];

    navigate(&heat_maps, &mut heats, Direction::Right, Position { x: 0, y: 0 }, 0, 0);

    heats.sort();
    let lowest_heat = heats[0];

    lowest_heat.to_string()   // Replace with the actual code here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // For test, run `cargo test --bin <folder name> to verify the test output`

        let input = include_str!("./input-test.txt");
        let result = process(input);
        assert_eq!(result, "102".to_string()); // Replace with the expected test output here
    }
}
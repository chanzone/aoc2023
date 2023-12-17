#[derive(Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    // When done, run `cargo run --bin <folder name> to get the output`

    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn beam_move(mirrors: &Vec<Vec<char>>, beams: &mut Vec<Vec<Vec<Direction>>>, pos: Position, dir: Direction) {
    if pos.x < 0 ||
        pos.y < 0 ||
        pos.x >= mirrors[0].len() as i32 ||
        pos.y >= mirrors.len() as i32 ||
        beams[pos.y as usize][pos.x as usize].contains(&dir)
    {
        return;
    }

    beams[pos.y as usize][pos.x as usize].push(dir);

    match mirrors[pos.y as usize][pos.x as usize] {
        '/' => {
            match dir {
                Direction::Up => beam_move(mirrors, beams, Position { x: pos.x + 1, y: pos.y }, Direction::Right),
                Direction::Down => beam_move(mirrors, beams, Position { x: pos.x - 1, y: pos.y }, Direction::Left),
                Direction::Left => beam_move(mirrors, beams, Position { x: pos.x, y: pos.y + 1 }, Direction::Down),
                Direction::Right => beam_move(mirrors, beams, Position { x: pos.x, y: pos.y - 1 }, Direction::Up),
            }
        },
        '\\' => {
            match dir {
                Direction::Up => beam_move(mirrors, beams, Position { x: pos.x - 1, y: pos.y }, Direction::Left),
                Direction::Down => beam_move(mirrors, beams, Position { x: pos.x + 1, y: pos.y }, Direction::Right),
                Direction::Left => beam_move(mirrors, beams, Position { x: pos.x, y: pos.y - 1 }, Direction::Up),
                Direction::Right => beam_move(mirrors, beams, Position { x: pos.x, y: pos.y + 1 }, Direction::Down),
            }
        },
        '|' => {
            match dir {
                Direction::Up => beam_move(mirrors, beams, Position { x: pos.x, y: pos.y - 1 }, Direction::Up),
                Direction::Down => beam_move(mirrors, beams, Position { x: pos.x, y: pos.y + 1 }, Direction::Down),
                Direction::Left |
                Direction::Right => {
                    beam_move(mirrors, beams, Position { x: pos.x, y: pos.y - 1 }, Direction::Up);
                    beam_move(mirrors, beams, Position { x: pos.x, y: pos.y + 1 }, Direction::Down);
                }
            }
        },
        '-' => {
            match dir {
                Direction::Left => beam_move(mirrors, beams, Position { x: pos.x - 1, y: pos.y }, Direction::Left),
                Direction::Right => beam_move(mirrors, beams, Position { x: pos.x + 1, y: pos.y }, Direction::Right),
                Direction::Up |
                Direction::Down => {
                    beam_move(mirrors, beams, Position { x: pos.x - 1, y: pos.y }, Direction::Left);
                    beam_move(mirrors, beams, Position { x: pos.x + 1, y: pos.y }, Direction::Right);
                }
            }
        },
        '.' => {
            match dir {
                Direction::Up => beam_move(mirrors, beams, Position { x: pos.x, y: pos.y - 1 }, Direction::Up),
                Direction::Down => beam_move(mirrors, beams, Position { x: pos.x, y: pos.y + 1 }, Direction::Down),
                Direction::Left => beam_move(mirrors, beams, Position { x: pos.x - 1, y: pos.y }, Direction::Left),
                Direction::Right => beam_move(mirrors, beams, Position { x: pos.x + 1, y: pos.y }, Direction::Right),
            }
        },
        _ => {
            panic!("Invalid character: {}", mirrors[pos.y as usize][pos.x as usize]);
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn process(input: &str) -> String {
    let mirrors: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut scores: Vec<usize> = vec![];

    for start_y in [0, mirrors.len() - 1].iter() {
        for start_x in 0..mirrors[0].len() {
            let mut beams: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; mirrors[0].len()]; mirrors.len()];
            let mut direction = Direction::Down;
            if start_y > &0 {
                direction = Direction::Up;
            }

            beam_move(&mirrors, &mut beams, Position { x: start_x as i32, y: *start_y as i32 }, direction);

            let mut score = 0;
            for y in 0..beams.len() {
                for x in 0..beams[y].len() {
                    if !beams[y][x].is_empty() {
                        score += 1;
                    }
                }
            }

            scores.push(score);
        }
    }

    scores.sort();
    scores.pop().unwrap().to_string()   // Replace with the actual code here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // For test, run `cargo test --bin <folder name> to verify the test output`

        let input = include_str!("./input-test.txt");
        let result = process(input);
        assert_eq!(result, "51".to_string()); // Replace with the expected test output here
    }
}
#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // When done, run `cargo run --bin <folder name> to get the output`

    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let galaxy: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut galaxy_expanded: Vec<Vec<char>> = galaxy.clone();
    let mut galaxy_expanded_positions: Vec<Point> = Vec::new();
    let empty_row = vec!['.'; galaxy[0].len()];

    let mut row_adjustment = 0;
    for (y, row) in galaxy.iter().enumerate() {
        let mut row_empty = true;
        for (_, &col) in row.iter().enumerate() {
            if col != '.' {
                row_empty = false;
                break;
            }
        }

        if row_empty {
            galaxy_expanded.insert(y + row_adjustment, empty_row.clone());
            row_adjustment += 1;
        }
    }

    let mut col_adjustment = 0;
    for (x, _) in galaxy[0].iter().enumerate() {
        let mut col_empty = true;
        for (_, row) in galaxy.iter().enumerate() {
            if row[x] != '.' {
                col_empty = false;
                break;
            }
        }

        if col_empty {
            for row in galaxy_expanded.iter_mut() {
                row.insert(x + col_adjustment, '.');
            }
            col_adjustment += 1;
        }
    }

    // dbg!(&galaxy_expanded);

    for (y, row) in galaxy_expanded.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            if col == '#' {
                galaxy_expanded_positions.push(Point { x: x as i32, y: y as i32 });
            }
        }
    }

    // dbg!(&galaxy_expanded_positions);

    let mut pairs: Vec<(Point, Point)> = Vec::new();
    for (i, pos1) in galaxy_expanded_positions.iter().enumerate() {
        for (j, pos2) in galaxy_expanded_positions.iter().enumerate() {
            let is_exists = pairs.iter().any(|(p1, p2)| {
                (p1.x == pos1.x && p1.y == pos1.y && p2.x == pos2.x && p2.y == pos2.y) ||
                (p1.x == pos2.x && p1.y == pos2.y && p2.x == pos1.x && p2.y == pos1.y)
            });
            if i != j && !is_exists {
                pairs.push((pos1.clone(), pos2.clone()));
            }
        }
    }

    // dbg!(&pairs);

    let pair_distances: i32 = pairs.iter().map(|(pos1, pos2)| {
        let distance: i32 = (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs();

        // dbg!(pos1);
        // dbg!(pos2);
        // dbg!(distance);

        distance
    }).sum();

    // dbg!(pair_distances);

    // dbg!(&galaxy_expanded);
    // dbg!(galaxy_expanded_positions.len());

    pair_distances.to_string()   // Replace with the actual code here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // For test, run `cargo test --bin <folder name> to verify the test output`

        let input = include_str!("./input-test.txt");
        let result = process(input);
        assert_eq!(result, "374".to_string()); // Replace with the expected test output here
    }
}
#[derive(Debug)]
struct Record {
    conditions: Vec<char>,
    damages: Vec<i32>,
}

fn main() {
    // When done, run `cargo run --bin <folder name> to get the output`

    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn generate_combinations(vector: &mut Vec<char>, index_start: usize, all_combinations: &mut Vec<Vec<char>>) {
    if index_start == vector.len() {
        all_combinations.push(vector.clone());
        return;
    }

    let mut index = index_start;
    while index < vector.len() && vector[index] != '?' {
        index += 1;
    }

    if index == vector.len() {
        all_combinations.push(vector.clone());
        return;
    }

    let mut vector_a = vector.clone();
    let mut vector_b = vector.clone();

    vector_a[index] = '.';
    generate_combinations(&mut vector_a, index + 1, all_combinations);

    vector_b[index] = '#';
    generate_combinations(&mut vector_b, index + 1, all_combinations);
}

fn process(input: &str) -> String {
    let records: Vec<Record> = input.lines().map(|line| {
        let mut record = line.trim().split_whitespace();
        Record {
            conditions: record.next().unwrap().chars().collect(),
            damages: record.next().unwrap().split(',').map(|x| x.parse().unwrap()).collect(),
        }
    }).collect();

    // dbg!(&records);

    let possible_counts: usize = records.iter().map(|record| {        
        let mut all_combinations: Vec<Vec<char>> = Vec::new();
        generate_combinations(&mut record.conditions.clone(), 0, &mut all_combinations);

        let possible_count = all_combinations.into_iter().filter(|combination| {
            let damages: String = combination.iter().collect::<String>();
            let damaged_groups: Vec<&str> = damages.split('.').collect::<Vec<_>>().into_iter().filter(|x| !x.is_empty()).collect();

            if damaged_groups.len() != record.damages.len() {
                return false;
            }

            for (index, group) in damaged_groups.iter().enumerate() {
                if group.len() != record.damages[index] as usize {
                    return false;
                }
            }

            return true;
        }).collect::<Vec<_>>().len();

        // dbg!(possible_count);

        possible_count
    }).into_iter().sum();

    // dbg!(possible_counts);

    possible_counts.to_string()   // Replace with the actual code here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // For test, run `cargo test --bin <folder name> to verify the test output`

        let input = include_str!("./input-test.txt");
        let result = process(input);
        assert_eq!(result, "21".to_string()); // Replace with the expected test output here
    }
}
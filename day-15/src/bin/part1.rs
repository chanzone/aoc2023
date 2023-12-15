fn main() {
    // When done, run `cargo run --bin <folder name> to get the output`

    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    input.replace('\n', "")
        .split(',')
        .map(|line| {
            let mut current_value: usize = 0;

            line.chars().for_each(|c| {
                current_value += c as usize;
                current_value *= 17;
                current_value %= 256;
            });

            current_value
        }).sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // For test, run `cargo test --bin <folder name> to verify the test output`

        let input = include_str!("./input-test.txt");
        let result = process(input);
        assert_eq!(result, "1320".to_string()); // Replace with the expected test output here
    }
}
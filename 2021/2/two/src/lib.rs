use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn get_values(input: &Vec<String>, direction: String) -> Vec<i64> {
    let filtered_input: Vec<&String> = input.iter().filter(|&s| s.contains(&direction)).collect();
    let mut output: Vec<i64> = vec![];
    for string in filtered_input {
        let split_string: Vec<&str> = string.split(" ").collect();
        output.push(split_string[1].parse().unwrap())
    }
    output
}

pub fn get_answer(input: &Vec<String>) -> i64 {
    let mut aim: i64 = 0;
    let mut depth: i64 = 0;
    let mut horizontal_position: i64 = 0;

    for string in input {
        let split_string: Vec<&str> = string.split(" ").collect();
        let direction = split_string[0];
        let value: i64 = split_string[1].parse().unwrap();

        calculate_totals(
            direction,
            &mut horizontal_position,
            value,
            &mut depth,
            &mut aim,
        );
    }
    horizontal_position * depth
}

fn calculate_totals(
    direction: &str,
    horizontal_position: &mut i64,
    value: i64,
    depth: &mut i64,
    aim: &mut i64,
) {
    if direction.eq("forward") {
        *horizontal_position += value;
        *depth += *aim * value;
    }
    if direction.eq("up") {
        *aim -= value;
    }
    if direction.eq("down") {
        *aim += value;
    }
}

pub fn get_file_data(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let values: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    values
}

pub fn sum(input: &Vec<i64>) -> i64 {
    return input.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extract_forward_values() {
        let input: Vec<String> = vec!["forward 3".to_string(), "down 10".to_string()];
        let expected: Vec<i64> = vec![3];
        let actual: Vec<i64> = get_values(&input, "forward".to_string());
        println!("actual: {:?}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn extract_down_values() {
        let input: Vec<String> = vec!["forward 3".to_string(), "down 10".to_string()];
        let expected: Vec<i64> = vec![10];
        let actual: Vec<i64> = get_values(&input, "down".to_string());
        println!("actual: {:?}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn extract_up_values() {
        let input: Vec<String> = vec!["forward 3".to_string(), "up 13".to_string()];
        let expected: Vec<i64> = vec![13];
        let actual: Vec<i64> = get_values(&input, "up".to_string());
        println!("actual: {:?}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sum_numbers() {
        let input = vec![1, 2, 3];
        let expected = 6;
        let actual = sum(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn do_part_one() {
        let input: Vec<String> = get_file_data("input.txt");

        let forward_values = get_values(&input, "forward".to_string());
        let down_values = get_values(&input, "down".to_string());
        let up_values = get_values(&input, "up".to_string());

        let forward_sum = sum(&forward_values);
        let down_sum = sum(&down_values);
        let up_sum = sum(&up_values);

        let net_vertical = down_sum - up_sum;
        let result = net_vertical * forward_sum;

        println!("forward_sum: {:?}", forward_sum);
        println!("down_sum: {:?}", down_sum);
        println!("up_sum: {:?}", up_sum);
        println!(
            "result: {:?} * {:?} = {:?}",
            net_vertical, forward_sum, result
        );
    }
    #[test]
    fn do_part_two() {
        let input: Vec<String> = get_file_data("input.txt");
        let result = get_answer(&input);
        println!("result: {}", result);
    }
}

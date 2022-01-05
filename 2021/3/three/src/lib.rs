use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::Chars;

static mut ones_global: Vec<i32> = vec![];
static mut zeros_global: Vec<i32> = vec![];

pub fn get_most_common_bits(input: &Vec<String>) -> Vec<String> {
    let mut ones: Vec<i32> = vec![];
    let mut zeros: Vec<i32> = vec![];
    let mut result_gamma = vec![];
    let mut result_epsilon = vec![];
    let length = input[0].len();
    let mut iterator = 0;

    while iterator < length {
        ones.push(0);
        zeros.push(0);
        result_gamma.push("");
        result_epsilon.push("");
        iterator += 1;
    }

    for number_string in input {
        for (i, char) in number_string.chars().enumerate() {
            if char == "1".parse().unwrap() {
                ones[i] += 1;
            } else {
                zeros[i] += 1
            };
        }
    }

    for (i, ones_value) in ones.iter().enumerate() {
        result_gamma[i] = if ones_value > &zeros[i] { "1" } else { "0" };
        result_epsilon[i] = if ones_value > &zeros[i] { "0" } else { "1" };
    }

    unsafe {
        ones_global = ones.clone();
        zeros_global = zeros.clone();
    }

    vec![result_gamma.join(""), result_epsilon.join("")]
}

pub fn get_file_data(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let values: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    values
}

pub fn calculate_result(gamma: &String, epsilon: &String) -> u32 {
    let gamma_converted = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon_converted = u32::from_str_radix(&epsilon, 2).unwrap();
    let result = gamma_converted * epsilon_converted;
    println!("{} converted to base 10 is : {:?}", gamma, gamma_converted);
    println!(
        "{} converted to base 10 is : {:?}",
        epsilon, epsilon_converted
    );
    println!(
        "{:?} * {:?} = {:?}",
        gamma_converted, epsilon_converted, result
    );
    result
}

pub fn get_oxygen(input: Vec<String>, ones: Vec<i32>, zeros: Vec<i32>) -> String {
    // let length = input.clone().len();
    // let input_clone = input.clone();
    for input_string in input.clone() {
        if input.len() > 1 {
            let chars = input_string.chars().collect::<Vec<_>>();
            let ith_char = chars[0];
            println!("initial input: {:?}", input);
            let filtered_input: Vec<String> = input.clone()
                .into_iter()
                .filter(|s| s.parse::<String>().unwrap().starts_with("1"))
                .collect();
            // println!("chars: {:?}", chars);
            // println!("filtered input: {:?}", filtered_input);
        } else {
            return input_string.clone();
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn most_common_bits() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();
        let expected = vec!["10110", "01001"];
        let actual = get_most_common_bits(&input);
        println!("actual result: {:?}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn do_part_three() {
        let input = get_file_data("input.txt");

        let gamma_and_epsilon_binary = get_most_common_bits(&input);

        let gamma = &gamma_and_epsilon_binary[0];
        let epsilon = &gamma_and_epsilon_binary[1];

        let result = calculate_result(gamma, epsilon);
        println!("result: {:?}", result);
        assert_eq!(3374136, result);
    }

    #[test]
    fn get_ones_and_zeros() {
        let input = get_file_data("input.txt");

        get_most_common_bits(&input);

        let ones;
        let zeros;

        unsafe {
            ones = ones_global.as_slice();
            zeros = zeros_global.as_slice();
        }
        println!("ones:  {:?} \nzeros: {:?}", ones, zeros);
    }

    #[test]
    fn do_second_part() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| s.parse::<String>().unwrap())
        .collect();

        get_most_common_bits(&input);

        let ones;
        let zeros;

        unsafe {
            ones = ones_global.to_vec();
            zeros = zeros_global.to_vec();
        }
        println!("ones:  {:?} \nzeros: {:?}", ones, zeros);

        let oxygen: String = get_oxygen(input.clone(), ones, zeros);
    }
}

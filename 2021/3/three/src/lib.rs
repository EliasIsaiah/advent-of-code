use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn get_most_common_bits(input: Vec<String>) -> Vec<String> {
    // let result: Vec<char> = vec![];
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
        if ones_value > &zeros[i] {
            result_gamma[i] = "1";
            result_epsilon[i] = "0";
        } else {
            result_gamma[i] = "0";
            result_epsilon[i] = "1";
        }
    }
    vec![result_gamma.join(""), result_epsilon.join("")]
}

pub fn get_file_data(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let values: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    values
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
        let actual = get_most_common_bits(input);
        println!("actual result: {:?}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn do_part_three() {
        let input = get_file_data("input.txt");

        let gamma_and_epsilon_binary = get_most_common_bits(input);

        let gamma = &gamma_and_epsilon_binary[0];
        let epsilon = &gamma_and_epsilon_binary[1];

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
    }
}

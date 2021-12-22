use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn get_number_of_depth_increases(depths: Vec<i64>) -> i64 {
    let mut number_of_depth_increases = 0;

    for (i, depth) in depths.iter().enumerate() {
        if i < depths.len() - 1 {
            let current_depth = depth;
            let next_depth = depths[i + 1];
            if current_depth < &next_depth {
                // println!("{} is less than {}", current_depth, &next_depth);
                number_of_depth_increases += 1;
            } // else {
              //     println!(
              //         "{} is greater than or equal to than {}",
              //         current_depth, &next_depth
              //     );
              // }
        }
    }
    number_of_depth_increases
}

pub fn get_file_data(file_path: &str) -> Vec<i64> {
    let file = File::open(file_path).expect("file asn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    numbers
}

pub fn generate_sliding_vector(depths: Vec<i64>) -> Vec<i64> {
    let mut sums = vec![];
    let depths_size = depths.len();

    for (i, depth) in depths.iter().enumerate() {
        if i + 2 >= depths_size {
            break;
        }
        let current_depth = depth;
        let next_depth = &depths[i + 1];
        let next_next_depth = &depths[i + 2];
        let sum = current_depth + next_depth + next_next_depth;
        // println!(
        //     "sum of {} + {} + {}: {}",
        //     current_depth, next_depth, next_next_depth, sum
        // );
        sums.push(sum);
    }
    sums
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_two_increases() {
        let v = vec![0, 1, 3, 2];
        // count the number of times a depth measurement increases from the previous measurement.
        let n = get_number_of_depth_increases(v);
        assert_eq!(2, n);
    }

    #[test]
    fn count_three_increases() {
        let v = vec![0, 1, 2, 3];
        // count the number of times a depth measurement increases from the previous measurement.
        let n = get_number_of_depth_increases(v);
        println!("number of depth increases: {}", n);
        assert_eq!(3, n);
    }

    #[test]
    fn count_all_increases() {
        let v = get_file_data("input.txt");
        let n = get_number_of_depth_increases(v);
        println!("number of depth increases: {}", n);
    }

    #[test]
    fn count_sliding_window_test_sample() {
        let v = vec![1, 2, 3, 4, 8];
        let s_v = generate_sliding_vector(v);
        let n = get_number_of_depth_increases(s_v);
        println!("number of depth increases sliding window style: {}", n);
        assert_eq!(2, n);
    }

    #[test]
    fn count_sliding_window_solution() {
        let v = get_file_data("input.txt");
        let s_v = generate_sliding_vector(v);
        let n = get_number_of_depth_increases(s_v);
        println!("number of depth increases sliding window style: {}", n);
    }
}

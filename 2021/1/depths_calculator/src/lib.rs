pub fn get_number_of_depth_increases(depths: Vec<i32>) -> i32 {
    let mut number_of_depth_increases = 0;

    for (i, depth) in depths.iter().enumerate() {
        if i < depths.len() - 1 {
            let current_depth = depth;
            let next_depth = depths[i + 1];
            if current_depth < &next_depth {
                number_of_depth_increases += 1;
            }
        }
    }
    number_of_depth_increases
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_two_increases() {
        let v = vec![0, 1, 3, 2];
        // count the number of times a depth measurement increases from the previous measurement.
        let n = get_number_of_depth_increases(v);
        assert_eq!(n, 2);
    }

    #[test]
    fn count_three_increases() {
        let v = vec![0, 1, 2, 3];
        // count the number of times a depth measurement increases from the previous measurement.
        let n = get_number_of_depth_increases(v);
        assert_eq!(n, 3);
    }
}

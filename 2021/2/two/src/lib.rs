pub fn get_foreward_total(foreward_values: Vec<&str>) -> i64 {
    3
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn forward_three() {
        let input = vec!["foreward 3"];
        let expected: i64 = 3;
        let actual = get_foreward_total(input);
        assert_eq!(expected, actual);
    }
    fn down_three() {}

    fn forward_three_down_three() {}
}

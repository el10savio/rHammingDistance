use itertools::zip_eq;

pub fn distance(left: String, right: String) -> Option<usize> {
    if left.len() != right.len() {
        return None;
    }

    Some(
        zip_eq(left.chars(), right.chars())
            .filter(|(l, r)| l != r)
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abcdef".to_string(), "ancdef".to_string(), Some(1); "base case")]
    #[test_case("abcdef".to_string(), "abcdef".to_string(), Some(0); "equal distance")]
    #[test_case("abcdef".to_string(), "abcd".to_string(), None; "unequal length")]
    fn test_distance(left: String, right: String, expected_result: Option<usize>) {
        let result = distance(left, right);
        assert_eq!(result, expected_result);
    }
}

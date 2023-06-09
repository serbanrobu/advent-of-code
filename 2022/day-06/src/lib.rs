use std::collections::HashSet;

pub fn part_1(input: &str) -> Option<usize> {
    marker_position(input, 4)
}

pub fn part_2(input: &str) -> Option<usize> {
    marker_position(input, 14)
}

fn marker_position(input: &str, marker_size: usize) -> Option<usize> {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(marker_size)
        .position(|w| HashSet::<&char>::from_iter(w).len() == marker_size)
        .map(|p| p + marker_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIGNAL: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_part_1() {
        let answer = part_1(SIGNAL);

        assert_eq!(answer, Some(7));
    }

    #[test]
    fn test_part_2() {
        let answer = part_2(SIGNAL);

        assert_eq!(answer, Some(19));
    }
}

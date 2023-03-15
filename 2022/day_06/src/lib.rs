use std::collections::HashSet;

pub fn solve_part_a(input: &str) -> String {
    let chars = input.chars().collect::<Vec<_>>();

    let mut out = 0;
    let mut i = 0;

    while i < chars.len() as u32 - 1 {
        let mut j = 0;
        let mut counts = HashSet::new();

        counts.insert(chars[i as usize]);

        while j < 4 {
            counts.insert(chars[(i + j) as usize]);
            j += 1
        }

        if counts.len() == 4 {
            out = i + 4;
            break;
        }

        i += 1
    }

    out.to_string()
}

// notes
//
//

// ============================================================================================================================== //
// ============================================================================================================================== //
// ============================================================================================================================== //

pub fn solve_part_b(input: &str) -> String {
    let chars = input.chars().collect::<Vec<_>>();

    let mut out = 0;
    let mut i = 0;

    while i < chars.len() as u32 - 1 {
        let mut j = 0;
        let mut counts = HashSet::new();

        counts.insert(chars[i as usize]);

        while j < 14 {
            counts.insert(chars[(i + j) as usize]);
            j += 1
        }

        if counts.len() == 14 {
            out = i + 14;
            break;
        }

        i += 1
    }

    out.to_string()
}

// notes
//
// vec has a `windows()` method i didn't know about that would
// have made this very easy

#[cfg(test)]
mod test {
    use super::{solve_part_a, solve_part_b};

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_part_a() {
        let result = solve_part_a(INPUT);

        assert_eq!(result, "7");
    }

    #[test]
    fn test_part_b() {
        let result = solve_part_b(INPUT);

        assert_eq!(result, "19");
    }
}

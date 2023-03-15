#![feature(iter_array_chunks)]

fn get_alphabet() -> impl Iterator<Item = char> {
    return ('a'..='z').chain('A'..='Z').into_iter();
}

pub fn solve_part_a(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let mut chars = get_alphabet();
            let (a, b) = l.split_at(l.len() / 2);
            let common = a.chars().filter(|c| b.contains(*c)).next().unwrap();

            return chars.position(|c| c == common).unwrap() + 1;
        })
        .sum::<usize>()
        .to_string()
}

// notes
//
// i could have done byte math (b'b' - 'b'a' gives you it's position)
// instead of creating the lookup string

// ============================================================================================================================== //
// ============================================================================================================================== //
// ============================================================================================================================== //

pub fn solve_part_b(input: &str) -> String {
    input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let common_item = a
                .chars()
                .find(|i| b.contains(*i) && c.contains(*i))
                .unwrap_or_else(|| panic!("Could not find common item"));

            get_alphabet()
                .position(|letter| letter == common_item)
                .unwrap()
                + 1
        })
        .sum::<usize>()
        .to_string()
}

// notes
//
//

#[cfg(test)]
mod test {
    use super::{solve_part_a, solve_part_b};

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_a() {
        let result = solve_part_a(INPUT);

        assert_eq!(result, "157");
    }

    #[test]
    fn test_part_b() {
        let result = solve_part_b(INPUT);

        assert_eq!(result, "70");
    }
}

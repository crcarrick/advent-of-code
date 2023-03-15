pub fn solve_part_a(input: &str) -> String {
    let elves = input.split("\n\n");

    let totals = elves.map(|elf| {
        return elf
            .split("\n")
            .map(|item| item.parse::<i32>().unwrap())
            .sum();
    });

    let mut most = 0;

    for total in totals {
        if total > most {
            most = total
        }
    }

    most.to_string()
}

// notes
//
//

// ============================================================================================================================== //
// ============================================================================================================================== //
// ============================================================================================================================== //

pub fn solve_part_b(input: &str) -> String {
    let mut elves = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();

    elves.sort();
    elves.reverse();

    elves.iter().take(3).sum::<u32>().to_string()
}

// notes
//
// i could have used .rev() on the iterator instead of collecting a vec

#[cfg(test)]
mod test {
    use super::{solve_part_a, solve_part_b};

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_a() {
        let result = solve_part_a(INPUT);

        assert_eq!(result, "24000");
    }

    #[test]
    fn test_part_b() {
        let result = solve_part_b(INPUT);

        assert_eq!(result, "45000");
    }
}

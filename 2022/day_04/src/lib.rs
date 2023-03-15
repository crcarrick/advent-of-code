pub fn solve_part_a(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let line = line
                .split(",")
                .flat_map(|g| g.split("-"))
                .collect::<Vec<_>>();

            let mut line: [[u8; 2]; 2] = match line[..] {
                [a, b, c, d] => [
                    [a.parse().unwrap(), b.parse().unwrap()],
                    [c.parse().unwrap(), d.parse().unwrap()],
                ],
                _ => panic!("malformed input"),
            };

            line.sort_by(|a, b| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));

            let [[a, b], [c, d]] = line;

            if a >= c && b <= d {
                return Some(());
            }

            return None;
        })
        .count()
        .to_string()
}

// notes
//
// i could have used .split_once() instead of the wacky pattern matching i did

// ============================================================================================================================== //
// ============================================================================================================================== //
// ============================================================================================================================== //

trait TupleMap<T, U> {
    fn map<F>(&self, f: F) -> (U, U, U, U)
    where
        F: Fn(&T) -> U;
}

impl<T, U> TupleMap<T, U> for (T, T, T, T) {
    fn map<F>(&self, f: F) -> (U, U, U, U)
    where
        F: Fn(&T) -> U,
    {
        let (a, b, c, d) = &self;

        return (f(a), f(b), f(c), f(d));
    }
}

pub fn solve_part_b(input: &str) -> String {
    input
        .lines()
        .filter(|l| {
            let (l, r) = l.split_once(",").unwrap();
            let ((a, b), (c, d)) = (l.split_once("-").unwrap(), r.split_once("-").unwrap());
            let (a, b, c, d) = (a, b, c, d).map(|i| i.parse::<u8>().unwrap());

            return a <= d && c <= b;
        })
        .count()
        .to_string()
}

// notes
//
// i want to learn about traits and goofed around with the TupleMap thing.
// i wouldn't have normally done that

#[cfg(test)]
mod test {
    use super::{solve_part_a, solve_part_b};

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_a() {
        let result = solve_part_a(INPUT);

        assert_eq!(result, "4");
    }

    #[test]
    fn test_part_b() {
        let result = solve_part_b(INPUT);

        assert_eq!(result, "4");
    }
}

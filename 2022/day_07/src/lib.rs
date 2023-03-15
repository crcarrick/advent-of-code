#![feature(map_try_insert)]

use std::collections::HashMap;

use regex::{Captures, Regex};

fn get_regexps() -> (Regex, Regex) {
    (
        Regex::new(r"cd (\S+)").unwrap(),
        Regex::new(r"(\d+) (\S+)").unwrap(),
    )
}

fn get_capture_at<'t>(c: Option<Captures<'t>>, i: usize) -> &str {
    c.unwrap().get(i).unwrap().as_str()
}

pub fn solve_part_a(input: &str) -> String {
    let (c_regex, f_regex) = get_regexps();

    let mut dir = vec![];
    let mut sizes = HashMap::new();

    input.lines().for_each(|l| {
        if f_regex.is_match(l) {
            let file_size = get_capture_at(f_regex.captures(l), 1)
                .parse::<u32>()
                .unwrap();

            (0..dir.len()).for_each(|i| {
                let path = dir.as_slice()[0..=i].join("/").to_string();

                sizes.entry(path).and_modify(|s| *s += file_size);
            });
        } else if c_regex.is_match(l) {
            let dirname = get_capture_at(c_regex.captures(l), 1);
            match dirname {
                ".." => {
                    dir.pop();
                }
                "/" => {
                    dir = vec!["root"];
                }
                _ => {
                    dir.push(dirname);
                }
            }

            let _ = sizes.try_insert(dir.join("/"), 0);
        }
    });

    sizes
        .values()
        .filter(|s| **s < 100000)
        .sum::<u32>()
        .to_string()
}

// notes
//
// this stumped me for a long time but is deceptively simple

// ============================================================================================================================== //
// ============================================================================================================================== //
// ============================================================================================================================== //

pub fn solve_part_b(input: &str) -> String {
    input.to_string()
}

// notes
//
//

#[cfg(test)]
mod test {
    use super::{solve_part_a, solve_part_b};

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_a() {
        let result = solve_part_a(INPUT);

        assert_eq!(result, "95437");
    }

    #[test]
    fn test_part_b() {
        let result = solve_part_b(INPUT);

        assert_eq!(result, "");
    }
}

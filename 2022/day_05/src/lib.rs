#![feature(iter_array_chunks)]
#![feature(iter_advance_by)]

use std::collections::VecDeque;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());

    let len = v[0].len();
    let mut iters = v.into_iter().map(|n| n.into_iter()).collect::<Vec<_>>();

    return (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect();
}

pub fn solve_part_a(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();

    let crates = crates.lines().map(|l| {
        l.chars()
            .enumerate()
            .filter(|&(idx, _)| idx != 0)
            .map(|(_, item)| item.to_string())
            .step_by(4)
            .collect::<Vec<_>>()
    });

    let mut stacks = transpose(crates.rev().collect())
        .into_iter()
        .map(|stack| stack.into_iter().filter(|x| x != " ").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let instructions = instructions
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|c| c.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for instruction in instructions {
        let (num, from, to) = match instruction[..] {
            [a, b, c] => (a, b, c),
            _ => panic!("Shit"),
        };

        (0..num).for_each(|_| {
            let temp = stacks[from - 1].pop().unwrap();

            stacks[to - 1].push(temp);
        })
    }

    stacks
        .iter()
        .map(|x| x.last().unwrap())
        .fold(String::new(), |a, b| a + b)
}

// notes
//
// i feel not so good about this solution

// ============================================================================================================================== //
// ============================================================================================================================== //
// ============================================================================================================================== //

#[derive(Debug)]
struct Crates {
    stacks: Vec<Vec<String>>,
}

impl Crates {
    fn from_str(s: &str) -> Crates {
        let stacks = s
            .lines()
            .map(|l| {
                l.chars()
                    .enumerate()
                    .filter(|&(i, _)| i != 0)
                    .map(|(_, c)| c.to_string())
                    .step_by(4)
                    .collect()
            })
            .collect();

        return Crates { stacks };
    }

    fn to_output(&self) -> String {
        return self
            .stacks
            .iter()
            .map(|i| i.last().unwrap())
            .fold(String::new(), |a, b| a + b);
    }

    fn rotate(&mut self) -> &mut Self {
        assert!(!self.stacks.is_empty());

        self.stacks.pop();
        self.stacks.reverse();

        // transpose
        self.stacks = self.stacks[0]
            .iter()
            .enumerate()
            .map(|(i, _)| {
                self.stacks
                    .iter()
                    .enumerate()
                    .map(|(j, _)| self.stacks[j][i].clone())
                    .collect()
            })
            .collect();

        return self;
    }

    fn trim(&mut self) -> &mut Self {
        for i in 0..self.stacks.len() {
            loop {
                let item = self.stacks[i].pop().unwrap();

                if item != " " {
                    self.stacks[i].push(item);
                    break;
                }
            }
        }

        return self;
    }

    fn swap(&mut self, instructions: &Instructions) -> &mut Self {
        assert!(!self.stacks.is_empty());

        for i in &instructions.instructions {
            let mut crates = VecDeque::new();

            for _ in 0..i.num {
                let tmp = self.stacks[i.from - 1].pop().unwrap();

                crates.push_front(tmp);
            }

            for c in crates {
                self.stacks[i.to - 1].push(c);
            }
        }

        return self;
    }
}

#[derive(Debug)]
struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_vector(v: Vec<usize>) -> Instruction {
        let (num, from, to) = match v[..] {
            [a, b, c] => (a, b, c),
            _ => panic!("Expected vector with 3 items but got {}", v.len()),
        };

        return Instruction { num, from, to };
    }
}

struct Instructions {
    instructions: Vec<Instruction>,
}

impl Instructions {
    fn from_str(s: &str) -> Instructions {
        return Instructions {
            instructions: s
                .lines()
                .map(|l| {
                    l.split_whitespace()
                        .filter_map(|i| i.parse::<usize>().ok())
                        .collect::<Vec<_>>()
                })
                .map(|l| Instruction::from_vector(l))
                .collect::<Vec<_>>(),
        };
    }
}

pub fn solve_part_b(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();

    Crates::from_str(crates)
        .rotate()
        .trim()
        .swap(&Instructions::from_str(instructions))
        .to_output()
}

// notes
//
// i added a bunch of data structures here / used method chaining etc
// to learn.  i also used some imperative code since i've mostly been using
// functional chains in the rest of the solutions

#[cfg(test)]
mod test {
    use super::{solve_part_a, solve_part_b};

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_a() {
        let result = solve_part_a(INPUT);

        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part_b() {
        let result = solve_part_b(INPUT);

        assert_eq!(result, "MCD");
    }
}

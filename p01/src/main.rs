#![feature(nll)]

extern crate aoclib;
use aoclib::*;

/*
impl Iterator for Thing {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
    }
}
*/

fn solve_a(input : &str) -> i32 {
    input.lines().fold(0, |sofar, line| {
        sofar + line.parse::<i32>().unwrap()
    })
}

fn solve_b(input : &str) -> u32 {
    0
}

fn main() {
    let input = read_all_stdin();
    //eprintln!("input: {}", input);

    if aoclib::should_solve_puzzle_a() {
        println!("answer: {}", solve_a(&input));
    } else {
        println!("answer: {}", solve_b(&input));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_1() {
        assert_eq!(solve_a("0"), 0);
        assert_eq!(solve_a("+1"), 1);
        assert_eq!(solve_a("-1"), -1);
        assert_eq!(solve_a("+1000"), 1000);
        assert_eq!(solve_a("-1000"), -1000);
        assert_eq!(solve_a("+1\n+2\n+3"), 6);
        assert_eq!(solve_a("-1\n-2\n-3"), -6);
        assert_eq!(solve_a("+1\n-2\n-3"), -4);
    }

    #[test]
    fn b_1() {
        assert_eq!(solve_b("+1\n-1"), 0);
        assert_eq!(solve_b("+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(solve_b("-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(solve_b("+7\n+7\n-2\n-7\n-4"), 14);
    }
}

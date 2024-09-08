#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parsing");
            }

            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed reading");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    #[allow(dead_code)]
    fn nexts<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        return (0..n).map(|_| self.next()).collect();
    }
}

fn solution() {
    let mut scanner = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    for _ in 0..scanner.next::<u64>() {
        let word = scanner.next::<String>();
        writeln!(
            out,
            "{}",
            match word {
                x if x.len() <= 10 => x,
                _ => format!(
                    "{}{}{}",
                    word.chars().nth(0).unwrap(),
                    word.len() - 2,
                    word.chars().last().unwrap()
                ),
            }
        )
        .ok();
    }
}

fn main() {
    solution();
}

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

    let v = scanner.nexts::<f64>(3);
    writeln!(
        out,
        "{}",
        ((v[0] / v[2]).ceil() * ((v[1] / v[2]).ceil())) as u64
    )
    .ok();
}

fn main() {
    solution();
}

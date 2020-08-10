use std::io;
use std::str;

// https://github.com/EbTech/rust-algorithms
pub struct Scanner<R: io::BufRead> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Scanner {
            reader,
            buffer: Vec::new(),
        }
    }
    pub fn next<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Token Parsing Failure");
            }
            let mut line = String::new();
            self.reader.read_line(&mut line).expect("Line Read Failure");
            self.buffer = line
                .split_ascii_whitespace()
                .rev()
                .map(String::from)
                .collect();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let n: u64 = scanner.next();

    let arr: Vec<u64> = (0..n).map(|_| scanner.next()).collect();
    
    let answer = arr[1..].iter().fold((arr[0], 0u64), |acc, x| {
        let diff = acc.0.checked_sub(*x).unwrap_or(0);
        if diff > 0 {
            (acc.0, acc.1 + diff)    
        } else {
            (*x, acc.1)
        }
    });

    println!("{}", answer.1);
}

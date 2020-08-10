use std::io;
use std::str;
use std::collections::HashSet;

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

    let set : HashSet<u64> = (1..n)
        .into_iter()
        .map(|_| scanner.next())
        .collect();
    

    for i in 1..=n {
        if set.get(&i).is_none() {
            println!("{}", i);
            break;
        }
    }
}

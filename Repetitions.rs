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

    let s: String = scanner.next();

    
    let mut last = 'X';
    let mut count = 1;
    let mut max_count = 1;

    for c in s.chars() {
        if c == last {
            count += 1
        } else {
            if count > max_count {
                max_count = count;
            }
            count = 1;
            last = c;
        }
    }
    if count > max_count {
        max_count = count;
    }
    println!("{}", max_count);

}

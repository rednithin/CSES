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

    let mut num: u64 = scanner.next();

    print!("{}", num);
    
    while num != 1 {
        if num % 2 == 0 {
            num = num / 2;
        } else {
            num = (num * 3) + 1;
        }

        print!(" {}", num);
    }
}

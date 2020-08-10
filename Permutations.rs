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
    let (half, offset) = if n % 2 == 0 {
        (n / 2, 0u64) 
    } else {
        (n / 2 + 1, 1) 
    };

    if n == 1 {
        println!("{}", 1);
    }
    else if n == 4 {
        println!("2 4 1 3 ");
    }
    else if n < 4 {
        println!("NO SOLUTION");
    } else {
        let mut answer = vec![];
        for (a, b) in (1..=half).zip(half+1..=n+offset) {
            answer.push(a.to_string());
            if b <= n {
                answer.push(b.to_string());
            }
        }

        println!("{}", answer.join(" "));
    }
}

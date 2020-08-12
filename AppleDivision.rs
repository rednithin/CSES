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
    let mut apples = vec![];

    let mut total_sum = 0;
    for _ in 0..n {
        let apple: i64 = scanner.next();
        apples.push(apple);
        total_sum += apple;
    }

    let mut answer = total_sum;
    for i in 0..1i64<<n {
        let subset_sum: i64 = apples
            .iter()
            .enumerate()
            .filter_map(|(j, item)| {
                if 1 << j & i > 0 {
                    Some(item)
                } else {
                    None
                }
            }).sum();
        let diff = (total_sum  - 2 * subset_sum ).abs();
        if diff < answer {
            answer = diff
        }
    }

    println!("{}", answer);
}

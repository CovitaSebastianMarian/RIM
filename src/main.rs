#![allow(unused)]
use std::io::{BufRead, Read};



struct Scanner {
    buffer: String,
}
impl Scanner {
    fn new() -> Scanner {
        Scanner {
            buffer: String::new(),
        }
    }
    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
    {
        while self.buffer.trim().is_empty() {
            std::io::stdin()
                .read_line(&mut self.buffer)
                .expect("Eroare la citirea din stdin!");
        }
        if let Some((a, b)) = self.buffer.clone().split_once(' ') {
            self.buffer = b.to_string();
            a.trim()
                .parse::<T>()
                .unwrap_or_else(|_| panic!("Eroare la parsare!"))
        } else {
            let result = self
                .buffer
                .trim()
                .parse::<T>()
                .unwrap_or_else(|_| panic!("Eroare la parsare!"));
            self.buffer.clear();
            result
        }
    }
    fn read_to_end_line(&mut self) -> String {
        let result = self.buffer.trim().to_string();
        self.buffer.clear();
        result
    }
}

struct FileScanner {
    reader: std::io::BufReader<std::fs::File>,
    buffer: String,
}
impl FileScanner {
    fn new(path: impl AsRef<std::path::Path>) -> FileScanner {
        let file = std::fs::File::open(path).expect("Eroare la deschiderea fișierului!");
        FileScanner {
            reader: std::io::BufReader::new(file),
            buffer: String::new(),
        }
    }
    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
    {
        if self.buffer.trim().is_empty() {
            self.buffer.clear();
            self.reader
                .read_line(&mut self.buffer)
                .expect("Eroare la citirea din fișier!");
        }
        if let Some((a, b)) = self.buffer.clone().split_once(' ') {
            self.buffer = b.to_string();
            a.trim()
                .parse::<T>()
                .unwrap_or_else(|_| panic!("Eroare la parsare!"))
        } else {
            let result = self
                .buffer
                .trim()
                .parse::<T>()
                .unwrap_or_else(|_| panic!("Eroare la parsare!"));
            self.buffer.clear();
            result
        }
    }
    fn read_to_end_line(&mut self) -> String {
        let result = self.buffer.trim().to_string();
        self.buffer.clear();
        result
    }
    fn read_to_end(&mut self) -> String {
        let mut result = String::new();
        self.reader
            .read_to_string(&mut result)
            .expect("Eroare la citirea întregului fișier!");
        result
    }
}


fn main() {
    let mut fscanner = FileScanner::new("adunare.in");
    let x: i32 = fscanner.read();
    let y: i32 = fscanner.read();

    println!("{}", x + y);
}
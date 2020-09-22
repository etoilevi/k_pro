use std::{
    fmt::Debug,
    io::{BufRead, BufReader, Read, Stdin},
    str::FromStr,
};

pub struct Reader<R: Read> {
    reader: BufReader<R>,
    words: Vec<String>,
}

impl<R: Read> Reader<R> {
    pub fn new(read: R) -> Self {
        Self {
            reader: BufReader::new(read),
            words: vec![],
        }
    }
    pub fn read<T>(&mut self) -> T
    where
        T: FromStr,
        T::Err: Debug,
    {
        self.ensure_words();
        self.words.pop().unwrap().parse().unwrap()
    }
    pub fn read_by<T>(&mut self, n: usize) -> Vec<T>
    where
        T: FromStr,
        T::Err: Debug,
    {
        (0..n).map(|_| self.read()).collect()
    }
    fn ensure_words(&mut self) {
        while self.words.is_empty() {
            let mut buf = String::new();
            self.reader.read_line(&mut buf).unwrap();
            let words = buf.split_ascii_whitespace();
            self.words = words.map(|s| s.to_string()).rev().collect();
        }
    }
}

impl Reader<Stdin> {
    pub fn from_stdin() -> Self {
        Self::new(std::io::stdin())
    }
}

#[test]
fn welcome_to_atcoder() {
    let input = r#"
    72
    128 256
    myonmyon
    "#;
    let mut rd = Reader::new(input.as_bytes());
    let a: u32 = rd.read();
    let b: u32 = rd.read();
    let c: u32 = rd.read();
    let s: String = rd.read();
    assert_eq!(a + b + c, 456);
    assert_eq!(&s, "myonmyon");
}

#[test]
fn read_array() {
    let input = r#"
    9
    1 2 3 4 5 6 7 8 9
    "#;
    let mut rd = Reader::new(input.as_bytes());
    let n: usize = rd.read();
    let a: Vec<u64> = rd.read_by(n);
    assert_eq!(n, 9);
    assert_eq!(a[0], 1);
    assert_eq!(a[8], 9);
    assert_eq!(a.iter().sum::<u64>(), 45);
}

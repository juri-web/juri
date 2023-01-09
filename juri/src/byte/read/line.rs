use std::{iter::Iterator, vec};

pub struct Line {
    bytes: Vec<u8>,
    _marker: usize,
}

impl Line {
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            _marker: 0,
        }
    }

    pub fn push(&mut self, bytes: &mut Vec<u8>) {
        self.bytes.append(bytes);
    }

    pub fn get_residue_bytes(mut self) -> Option<Vec<u8>> {
        if self.bytes.is_empty() {
            None
        } else {
            let bytes = self.bytes.drain(..).collect();
            self._marker = 0;
            Some(bytes)
        }
    }
}

impl Iterator for Line {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut flag_n = false;
        let mut flag_r = false;

        let mut bytes_iter = self.bytes[self._marker..].iter().enumerate();
        loop {
            if let Some((index, value)) = bytes_iter.next() {
                if flag_r {
                    if *value == 10 {
                        flag_n = true;
                    } else {
                        flag_r = false;
                    }
                }
                if *value == 13 {
                    flag_r = true;
                }

                if flag_n && flag_r {
                    let bytes: Vec<u8> = self.bytes.drain(..(index - 1)).collect();
                    self.bytes.drain(..2);
                    self._marker = 0;
                    break Some(bytes);
                }
            } else {
                self._marker = self.bytes.len();
                if self.bytes.last() == Some(&13) {
                    self._marker -= 1;
                }
                break None;
            }
        }
    }
}

#[test]
fn test_newline() {
    let mut line = Line::new();
    let mut bytes: Vec<u8> = vec![1, 2, 13, 10];
    line.push(&mut bytes);

    assert_eq!(line.next(), Some(vec![1, 2]));
    assert_eq!(line.next(), None);
    assert_eq!(line.get_residue_bytes(), None);

    let mut line = Line::new();
    let mut bytes: Vec<u8> = vec![1, 2, 13, 10, 2, 3, 13, 10];
    line.push(&mut bytes);

    assert_eq!(line.next(), Some(vec![1, 2]));
    assert_eq!(line.next(), Some(vec![2, 3]));
    assert_eq!(line.next(), None);
    assert_eq!(line.get_residue_bytes(), None);

    let mut line = Line::new();
    let mut bytes: Vec<u8> = vec![1, 2, 13, 10, 2, 3, 13, 10, 13, 10];
    line.push(&mut bytes);

    assert_eq!(line.next(), Some(vec![1, 2]));
    assert_eq!(line.next(), Some(vec![2, 3]));
    assert_eq!(line.next(), Some(vec![]));
    assert_eq!(line.next(), None);
    assert_eq!(line.get_residue_bytes(), None);

    let mut line = Line::new();
    let mut bytes: Vec<u8> = vec![1, 2, 13];
    line.push(&mut bytes);

    assert_eq!(line.next(), None);
    let mut bytes: Vec<u8> = vec![10];
    line.push(&mut bytes);
    assert_eq!(line.next(), Some(vec![1, 2]));
    assert_eq!(line.next(), None);
    assert_eq!(line.get_residue_bytes(), None);

    let mut line = Line::new();
    let mut bytes: Vec<u8> = vec![1, 2, 13, 10];
    line.push(&mut bytes);
}

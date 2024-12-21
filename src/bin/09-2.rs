use std::io::{read_to_string, stdin};

use itertools::Itertools;

struct Segment {
    length: usize,
    file: Option<usize>,
}

impl Segment {
    fn is_file(&self) -> bool {
        self.file.is_some()
    }

    fn fit(&mut self, size: usize) -> bool {
        if self.file.is_some() {
            return false;
        }
        let Some(new_length) = self.length.checked_sub(size) else {
            return false;
        };
        self.length = new_length;
        true
    }

    fn checksum(&self, i: usize) -> usize {
        (i..i + self.length).sum::<usize>() * self.file.unwrap_or_default()
    }
}

fn main() {
    let mut disk = read_to_string(stdin())
        .unwrap()
        .trim_ascii_end()
        .chars()
        .map(|c| (c as usize) - ('0' as usize))
        .enumerate()
        .map(|(i, length)| Segment {
            length,
            file: (i % 2 == 0).then_some(i / 2),
        })
        .collect_vec();
    let mut i = disk.len() - 1;
    'outer: while i > 0 {
        if disk[i].is_file() {
            let length = disk[i].length;
            for j in 0..i {
                if disk[j].fit(length) {
                    let file = disk[i].file.take();
                    let segment = Segment { length, file };
                    disk.insert(j, segment);
                    continue 'outer;
                }
            }
        }
        i -= 1;
    }
    let (_, result) = disk.into_iter().fold((0, 0), |(i, result), y| {
        (i + y.length, result + y.checksum(i))
    });
    println!("{result}");
}

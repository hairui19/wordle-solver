use std::collections::HashSet;

const DICTIONARY: &str = include_str!("../../dictionary.txt");

#[derive(Debug)]
pub struct Naive {
    remaining: HashSet<&'static str>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining: HashSet::from_iter(
                DICTIONARY.lines().map(|str| str.split_once(" ").unwrap().0),
            ),
        }
    }

    pub fn trim(&mut self, )
}

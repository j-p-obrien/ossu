use std::{
    fs,
    num::ParseIntError,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Code(u32);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CodeBucket(Vec<Vec<Code>>);

impl Code {
    pub fn hamming_distance(&self, other: &Code) -> usize {
        (self.0 ^ other.0).count_ones() as usize
    }
}

impl Deref for Code {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Code {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binary_string: String = s.split_whitespace().collect();
        let code = u32::from_str_radix(&binary_string, 2)?;
        Ok(Code(code))
    }
}

#[derive(Debug)]
pub enum ParseBucketError {
    FormatError(&'static str),
    ParseError(ParseIntError),
}

impl From<ParseIntError> for ParseBucketError {
    fn from(value: ParseIntError) -> Self {
        ParseBucketError::ParseError(value)
    }
}

impl FromStr for CodeBucket {
    type Err = ParseBucketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_iter = s.lines();
        let code_length: usize = line_iter
            .next()
            .ok_or(Self::Err::FormatError("String is empty"))?
            .split_whitespace()
            .nth(1)
            .ok_or(Self::Err::FormatError("Expected two numbers on first line"))?
            .parse()?;

        let mut buckets = vec![vec![]; code_length + 1];
        for line in line_iter {
            let code = Code::from_str(line)?;
            buckets[code.0.count_ones() as usize].push(code);
        }
        Ok(CodeBucket(buckets))
    }
}

impl Deref for CodeBucket {
    type Target = Vec<Vec<Code>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CodeBucket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CodeBucket {
    pub fn from_file(path: &str) -> CodeBucket {
        let file_data = fs::read_to_string(path).unwrap();
        CodeBucket::from_str(&file_data).expect("Couldn't read from file")
    }

    pub fn cluster(&self, spacing: usize) -> usize {
        let mut buckets = self.clone();
        let mut clusters = vec![];

        for i in 0..buckets.len() {
            while let Some(code) = buckets[i].pop() {
                clusters.push(buckets.remove_all_neighbors(&code, spacing));
            }
            //dbg!(clusters.len());
        }

        clusters.len()
    }

    fn remove_all_neighbors(&mut self, code: &Code, spacing: usize) -> Vec<Code> {
        let mut neighbors = vec![*code];
        let mut todo = neighbors.clone();
        while let Some(code) = todo.pop() {
            let code_bucket = code.count_ones() as usize;
            let buckets_to_check = (code_bucket - 2)..=(code_bucket + 2);
            for bucket in buckets_to_check {
                let bucket_neighbors = self.take_neighbors_from_bucket(&code, bucket, spacing);
                neighbors.extend_from_slice(&bucket_neighbors);
                todo.extend_from_slice(&bucket_neighbors);
            }
        }
        neighbors
    }

    fn take_neighbors_from_bucket(
        &mut self,
        code: &Code,
        bucket: usize,
        spacing: usize,
    ) -> Vec<Code> {
        let neighbor_indices: Vec<_> = self[bucket]
            .iter()
            .enumerate()
            .filter(|(i, other)| code.hamming_distance(other) < spacing)
            .map(|(i, other)| i)
            .collect();

        self.remove_indices_from_bucket(bucket, neighbor_indices)
    }

    fn remove_indices_from_bucket(
        &mut self,
        bucket: usize,
        neighbor_indices: Vec<usize>,
    ) -> Vec<Code> {
        let mut neighbors = Vec::with_capacity(neighbor_indices.len());
        for i in neighbor_indices.iter().rev() {
            neighbors.push(self[bucket].swap_remove(*i))
        }
        neighbors
    }
}

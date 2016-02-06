//! Provides utilities for interaction with hashing algorithms

extern crate crypto;

use crypto::digest::Digest;

///Represents checksum algorithm
pub struct Checksum {
    name: String,
    algo: Box<Digest>
}

impl Checksum {
    ///Constructs new ```Checksum```
    pub fn new<T: 'static + Digest>(algo_name: String, algorithm: T) -> Checksum {
        Checksum {
            name: algo_name,
            algo: Box::new(algorithm)
        }
    }

    #[inline]
    ///Provides input for algorithm
    pub fn input(&mut self, slice_content: &[u8]) {
        self.algo.input(slice_content);
    }

    #[inline]
    ///Returns result in the following format: ```{name} - {hash}```
    pub fn result(&mut self) -> String {
        format!("{:8} - {}", self.name, self.algo.result_str())
    }

    #[inline(always)]
    ///Returns hashsum
    pub fn checksum(&mut self) -> String {
        self.algo.result_str()
    }

    #[inline(always)]
    ///Resets hashing
    pub fn reset(&mut self) {
        self.algo.reset();
    }

    #[inline]
    ///Returns algorithm's name for file extension(lowercase)
    pub fn get_file_ext(&self) -> String {
        self.name[0..self.name.len()-1].chars()
                                       .map(|elem| elem.to_lowercase().next().unwrap())
                                       .collect()
    }

    #[inline(always)]
    ///Returns name of algorithm in format ```{name} -```
    pub fn get_type_string(&self) -> String {
        format!("{:8} - ", self.name)
    }
}

impl PartialEq for Checksum {
    fn eq(&self, right: &Checksum) -> bool {
        self.name == right.name
    }

    fn ne(&self, right: &Checksum) -> bool {
        self.name != right.name
    }
}


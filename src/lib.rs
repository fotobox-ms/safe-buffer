#![feature(linked_list_cursors)]
#![feature(allocator_api)]

use std::collections::LinkedList;
use std::io;
use std::io::{Error, ErrorKind, SeekFrom};
use std::alloc::Allocator;


include!("buffer.rs");
include!("chunk.rs");
include!("list.rs");

#[cfg(test)]
mod tests {
    use std::io::Write;
    use rand::{random, Rng};
    use super::*;

    #[test]
    fn random_write() {
        _random_write(0);
        _random_write(10);
        _random_write(3048);
        _random_write(CHUNK_SIZE + 1);
        _random_write(10_000);
        _random_write(1_000_000);
    }

    fn _random_write(len: usize) {
        let mut rng = rand::thread_rng();

        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(rng.gen_range(0..u8::MAX));
        }

        let mut buffer = Buffer::new();
        buffer.write(data.as_slice()).expect("write failed");
    }
}
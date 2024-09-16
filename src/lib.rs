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
    use std::io::{Seek, Write};
    use rand::{Rng};
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;
    use super::*;

    #[test]
    fn random_write() {
        _random_write_and_read(0);
        _random_write_and_read(10);
        _random_write_and_read(3048);
        _random_write_and_read(CHUNK_SIZE + 1);
        _random_write_and_read(10_000);
        _random_write_and_read(1_000_000);
    }

    fn _random_write_and_read(len: usize) {
        // write
        let mut rng = rand::thread_rng();
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(rng.gen_range(0..u8::MAX));
        }

        let mut buffer = Buffer::new();
        buffer.write(data.as_slice()).expect("write failed");

        // read
        let mut ret = Vec::with_capacity(len);
        while let Some(chunk) = buffer.pop_front() {
            for b in chunk.as_slice() {
                ret.push(*b);
            }
        }

        assert_eq!(ret, data);
    }

    fn _prepare(len: usize) -> Buffer {
        let mut rng = rand::thread_rng();

        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(rng.gen_range(0..u8::MAX));
        }

        let mut buffer = Buffer::new();
        buffer.write(data.as_slice()).expect("write failed");

        buffer
    }

    #[test]
    fn seek_end() {
        let mut buffer = _prepare(1_000_000);
        buffer.seek(SeekFrom::End(0)).expect("seek failed");
    }

    #[test]
    fn zip_test() {
        let mut buf: Buffer = Buffer::new();
        let mut zip = ZipWriter::new(&mut buf);

        let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

        fn fetch_img(img_url: &str) -> Result<Vec<u8>, reqwest::Error> {
            let res = reqwest::blocking::get(img_url)?;
            let data = res.bytes()?;
            let data_vec = data.to_vec();

            Ok(data_vec)
        }

        let data = &fetch_img("https://cdn.esawebb.org/archives/images/screen/weic2216b.jpg").expect("failed to fetch image");

        for i in 0..500 {
            let name = format!("img{i}.jpg");

            zip.start_file(name, options).expect("Failed to start file");
            zip.write(&data).expect("write failed");
        };

        zip.finish().expect("finish failed");
    }
}
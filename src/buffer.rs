use std::collections::LinkedList;
use std::{io};
use std::io::{Error, ErrorKind, SeekFrom};
use crate::chunk::{Chunk, CHUNK_SIZE};
use crate::list::MoveTo;

pub struct Buffer {
    chunks: LinkedList<Chunk>,
    pos: usize,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            chunks: LinkedList::new(),
            pos: 0,
        }
    }

    fn grow_to(&mut self, index: usize) -> io::Result<()> {
        let chunk_index = index / CHUNK_SIZE;

        // check if already allocated
        let capacity = self.chunks.len();
        if chunk_index < capacity {
            return Ok(())
        }

        for _ in capacity..chunk_index {
            self.chunks.push_back(Chunk::new()?);
        }

        Ok(())
    }

    pub fn len(&mut self) -> usize {
        if self.chunks.is_empty() { 0 } else {
            let last = self.chunks.back().unwrap();
            (self.chunks.len() - 1) * CHUNK_SIZE + last.len()
        }
    }

    pub fn pop_front(&mut self) -> Option<Chunk> {
        self.pos = if self.pos > CHUNK_SIZE {self.pos - CHUNK_SIZE} else { 0 };
        self.chunks.pop_front()
    }
}

impl io::Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // ensure size
        self.grow_to(self.pos + buf.len() - 1)?;

        let mut iter = self.chunks.cursor_back_mut();

        for i in 0..buf.len() {
            let cursor = self.pos + i;
            let chunk_index = cursor / CHUNK_SIZE;
            let byte_index = cursor % CHUNK_SIZE;

            iter.move_to(chunk_index);
            iter.current().as_mut().unwrap().set(byte_index, buf[i])
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}


impl io::Seek for Buffer {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let offset = match pos {
            SeekFrom::Start(offset) => offset as i64,
            SeekFrom::End(n) => self.len() as i64 + n,
            SeekFrom::Current(n) => self.pos as i64 + n
        };

        if offset < 0 {
            return Err(Error::new(ErrorKind::InvalidInput, "Seeking before start of buffer!"));
        }

        let trunc = offset as usize;
        if trunc as i64 != offset {
            return Err(Error::new(ErrorKind::OutOfMemory, "Tried seeking past the memory limit!"));
        }

        self.pos = offset as usize;
        Ok(self.pos as u64)
    }
}

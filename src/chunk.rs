pub const CHUNK_SIZE: usize = 1 << 12;

pub struct Chunk {
    inner: Vec<u8>
}

impl Chunk {
    pub fn new() -> io::Result<Self> {
        let mut vec = Vec::new();
        match vec.try_reserve(CHUNK_SIZE) {
            Ok(_) => Ok(Chunk { inner: vec }),
            Err(_) => Err(io::Error::new(ErrorKind::OutOfMemory, "Could not allocate new chunk!")),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn set(&mut self, index: usize, value: u8) {
        assert!(index < CHUNK_SIZE);
        self.inner[index] = value;
    }

    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }
}

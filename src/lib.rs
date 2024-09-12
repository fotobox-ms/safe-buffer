#![feature(linked_list_cursors)]
#![feature(allocator_api)]

use std::collections::LinkedList;
use std::io;
use std::io::{Error, ErrorKind, SeekFrom};
use std::alloc::Allocator;


include!("buffer.rs");
include!("chunk.rs");
include!("list.rs");

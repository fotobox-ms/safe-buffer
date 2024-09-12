use std::alloc::{Allocator};

pub trait MoveTo {
    fn move_to(&mut self, pos: usize);
}

impl<'a, T: 'a, A: Allocator> MoveTo for std::collections::linked_list::CursorMut<'a, T, A> {
    fn move_to(&mut self, pos: usize) {
        let diff = self.index().unwrap() as isize - pos as isize;
        if diff == 0 {
            return;
        } else if diff < 0 {
            for _ in 0..diff {
                self.move_next()
            }
        } else {
            for _ in diff..0 {
                self.move_prev()
            }
        }

        assert_eq!(self.index().unwrap(), pos)
    }
}

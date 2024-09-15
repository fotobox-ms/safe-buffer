pub trait MoveTo {
    fn move_to(&mut self, pos: usize);
}

impl<'a, T: 'a, A: Allocator> MoveTo for std::collections::linked_list::CursorMut<'a, T, A> {
    fn move_to(&mut self, pos: usize) {
        // if we are on the ghost element, find the correct end to start at
        let current = match self.index() {
            Some(i) => i,
            None => {
                let length = self.as_list().len();
                if length == 0 { return; }

                // choose where to start
                if pos < length - pos {
                    self.move_next();
                    0
                } else {
                    self.move_prev();
                    length - 1
                }
            }
        };

        let diff = current as isize - pos as isize;
        if diff == 0 { return; }

        // we always reach the correct index, since the cursor is circular
        // however we try to move in the correct direction to reduce calls!
        while self.index() != Some(pos) {
            #[cfg(test)]
            {
                let i = self.index();
                if i.is_none() { print!("X ")}
                else { print!("{} ", i.unwrap()) }
            }

            if diff < 0 {
                self.move_next();
            } else {
                self.move_prev();
            }
        }

        println!();
        assert_eq!(self.index().unwrap(), pos);
    }
}

use std::ops::Range;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Selection {
    pub start: usize,
    pub end: usize,
    pub reversed: bool,
}

impl Selection {
    pub fn caret(offset: usize) -> Self {
        Self {
            start: offset,
            end: offset,
            reversed: false,
        }
    }

    pub fn range(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
            reversed: false,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn reversed(&self) -> bool {
        self.reversed
    }

    pub fn cursor(&self) -> usize {
        if self.reversed { self.start } else { self.end }
    }

    pub fn range_ref(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn set_caret(&mut self, offset: usize) {
        self.start = offset;
        self.end = offset;
        self.reversed = false;
    }

    pub fn select_to(&mut self, offset: usize) {
        if self.reversed {
            self.start = offset;
        } else {
            self.end = offset;
        }

        if self.end < self.start {
            self.reversed = !self.reversed;
            std::mem::swap(&mut self.start, &mut self.end);
        }
    }

    pub fn set_range(&mut self, range: Range<usize>) {
        self.start = range.start;
        self.end = range.end;
        self.reversed = false;
    }
}

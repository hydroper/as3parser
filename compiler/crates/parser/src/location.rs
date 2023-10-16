use std::rc::Rc;
use std::cmp::Ordering;
use crate::source::Source;

/// Represents a source location. This location includes
/// spanning lines and columns and the reference source.
#[derive(Clone)]
pub struct Location {
    /// The source file that this location belongs to.
    pub source: Rc<Source>,

    /// First line number, counted from one.
    pub first_line_number: usize,

    /// Last line number, counted from one.
    pub last_line_number: usize,

    pub first_offset: usize,

    pub last_offset: usize,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source) &&
            self.first_line_number == other.first_line_number &&
            self.last_line_number == other.last_line_number &&
            self.first_offset == other.first_offset &&
            self.last_offset == other.last_offset
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.first_offset.partial_cmp(&other.first_offset)
    }
}

impl Location {
    /// Zero based first column of the location in code points.
    pub fn first_column(&self) -> usize {
        let line_offset = *self.source.line_number_offsets.borrow().get(self.first_line_number).unwrap_or(&0);
        let target_offset = self.first_offset;
        if line_offset > target_offset {
            return 0;
        }
        let mut i = 0;
        for _ in self.source.text[line_offset..target_offset].chars() {
            i += 1;
        }
        i
    }

    /// Zero based last column of the location in code points.
    pub fn last_column(&self) -> usize {
        let line_offset = *self.source.line_number_offsets.borrow().get(self.last_line_number).unwrap_or(&0);
        let target_offset = self.last_offset;
        if line_offset > target_offset {
            return 0;
        }
        let mut i = 0;
        for _ in self.source.text[line_offset..target_offset].chars() {
            i += 1;
        }
        i
    }
}
use std::cmp::Ordering;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use std::rc::Rc;
use crate::source::Source;

/// Represents a source location. This location includes
/// spanning lines and columns and the reference source.
#[derive(Clone, Serialize, Deserialize)]
pub struct Location {
    /// The source file that this location belongs to.
    #[serde(skip)]
    pub(crate) source: Rc<Source>,

    /// First line number, counted from one.
    #[serde(skip)]
    pub(crate) first_line_number: usize,

    /// Last line number, counted from one.
    #[serde(skip)]
    pub(crate) last_line_number: usize,

    /// First UTF-8 offset.
    #[serde(skip)]
    pub(crate) first_offset: usize,

    /// Last UTF-8 offset.
    #[serde(skip)]
    pub(crate) last_offset: usize,
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Location(\n\
                first_line_number={},\n\
                first_column={},\n\
                first_offset={},\n\
                last_line_number={},\n\
                last_column={}\n\
                last_offset={}\n\
            )",
            self.first_line_number,
            self.first_column(),
            self.first_offset,
            self.last_line_number,
            self.last_column(),
            self.last_offset
        )
    }
}

impl Eq for Location {}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source) &&
            self.first_line_number == other.first_line_number &&
            self.last_line_number == other.last_line_number &&
            self.first_offset == other.first_offset &&
            self.last_offset == other.last_offset
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.first_offset.partial_cmp(&other.first_offset)
    }
}

impl Location {
    /// Builds a location.
    pub fn with_lines_and_offsets(
        source: &Rc<Source>,
        first_line_number: usize,
        last_line_number: usize,
        first_offset: usize,
        last_offset: usize,
    ) -> Self {
        Self {
            source: source.clone(),
            first_line_number,
            last_line_number,
            first_offset,
            last_offset,
        }
    }

    /// Builds a location.
    pub fn with_line_and_offsets(
        source: &Rc<Source>,
        line_number: usize,
        first_offset: usize,
        last_offset: usize,
    ) -> Self {
        Self::with_lines_and_offsets(source, line_number, line_number, first_offset, last_offset)
    }

    /// Builds a location.
    pub fn with_line_and_offset(source: &Rc<Source>, line_number: usize, offset: usize) -> Self {
        Self::with_lines_and_offsets(source, line_number, line_number, offset, offset)
    }

    /// Build a location by combining two locations. `self`
    /// serves as the first location, while `other` serves as the
    /// last location.
    pub fn combine_with(&self, other: Location) -> Self {
        Self {
            source: self.source.clone(),
            first_line_number: self.first_line_number,
            last_line_number: other.last_line_number,
            first_offset: self.first_offset,
            last_offset: other.last_offset,
        }
    }

    /// Build a location by combining two locations. `self`
    /// serves as the first location, while the first column and first line
    /// of `other` serves as the last location.
    pub fn combine_with_start_of(&self, other: Location) -> Self {
        Self {
            source: self.source.clone(),
            first_line_number: self.first_line_number,
            last_line_number: other.first_line_number,
            first_offset: self.first_offset,
            last_offset: other.first_offset,
        }
    }

    /// The source file that this location belongs to.
    pub fn source(&self) -> Rc<Source> {
        self.source.clone()
    }

    /// First line number, counted from one.
    pub fn first_line_number(&self) -> usize {
        self.first_line_number
    }

    /// Last line number, counted from one.
    pub fn last_line_number(&self) -> usize {
        self.first_line_number
    }

    // The first byte offset of this location.
    pub fn first_offset(&self) -> usize {
        self.first_offset
    }

    // The last byte offset of this location.
    pub fn last_offset(&self) -> usize {
        self.last_offset
    }

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

    pub fn character_count(&self) -> usize {
        self.source.text[self.first_offset..self.last_offset].chars().count()
    }
    
    /// Indicates whether a previous location and a next location
    /// have a line break in between.
    pub fn line_break(&self, other: &Self) -> bool {
        self.last_line_number != other.first_line_number
    }
}
use std::cmp::Ordering;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use std::rc::Rc;
use crate::compilation_unit::*;

/// Represents a source location. This location includes
/// spanning lines and columns and the reference compilation unit.
#[derive(Clone, Serialize, Deserialize)]
pub struct Location {
    /// The compilation unit that this location belongs to.
    #[serde(skip)]
    pub(crate) compilation_unit: Rc<CompilationUnit>,

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
            "Location(first_line_number={}, first_column={}, first_offset={}, last_line_number={}, last_column={}, last_offset={})",
            self.first_line_number(),
            self.first_column(),
            self.first_offset,
            self.last_line_number(),
            self.last_column(),
            self.last_offset
        )
    }
}

impl Eq for Location {}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.compilation_unit, &other.compilation_unit) &&
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
    pub fn with_offsets(
        compilation_unit: &Rc<CompilationUnit>,
        first_offset: usize,
        last_offset: usize,
    ) -> Self {
        Self {
            compilation_unit: compilation_unit.clone(),
            first_offset,
            last_offset,
        }
    }

    /// Builds a location.
    pub fn with_offset(compilation_unit: &Rc<CompilationUnit>, offset: usize) -> Self {
        Self::with_offsets(compilation_unit, offset, offset)
    }

    /// Build a location by combining two locations. `self`
    /// serves as the first location, while `other` serves as the
    /// last location.
    pub fn combine_with(&self, other: Location) -> Self {
        Self {
            compilation_unit: self.compilation_unit.clone(),
            first_offset: self.first_offset,
            last_offset: other.last_offset,
        }
    }

    /// Build a location by combining two locations. `self`
    /// serves as the first location, while the first column and first line
    /// of `other` serve as the last location.
    pub fn combine_with_start_of(&self, other: Location) -> Self {
        Self {
            compilation_unit: self.compilation_unit.clone(),
            first_offset: self.first_offset,
            last_offset: other.first_offset,
        }
    }

    /// The compilation unit that this location belongs to.
    pub fn compilation_unit(&self) -> Rc<CompilationUnit> {
        self.compilation_unit.clone()
    }

    /// First line number, counted from one.
    pub fn first_line_number(&self) -> usize {
        self.compilation_unit.get_line_number(self.first_offset)
    }

    /// Last line number, counted from one.
    pub fn last_line_number(&self) -> usize {
        self.compilation_unit.get_line_number(self.last_offset)
    }

    /// First line offset, counted from one.
    pub fn first_line_offset(&self) -> usize {
        self.compilation_unit.get_line_offset_from_offset(self.first_offset())
    }

    /// Last line offset, counted from one.
    pub fn last_line_offset(&self) -> usize {
        self.compilation_unit.get_line_offset_from_offset(self.last_offset())
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
        let line_offset = self.first_line_offset();
        let target_offset = self.first_offset;
        if line_offset > target_offset {
            return 0;
        }
        let mut i = 0;
        for _ in self.compilation_unit.text[line_offset..target_offset].chars() {
            i += 1;
        }
        i
    }

    /// Zero based last column of the location in code points.
    pub fn last_column(&self) -> usize {
        let line_offset = self.last_line_offset();
        let target_offset = self.last_offset;
        if line_offset > target_offset {
            return 0;
        }
        let mut i = 0;
        for _ in self.compilation_unit.text[line_offset..target_offset].chars() {
            i += 1;
        }
        i
    }

    pub fn character_count(&self) -> usize {
        self.compilation_unit.text[self.first_offset..self.last_offset].chars().count()
    }

    /// Indicates whether a previous location and a next location
    /// have a line break in between.
    pub fn line_break(&self, other: &Self) -> bool {
        self.last_line_number() != other.first_line_number()
    }
}
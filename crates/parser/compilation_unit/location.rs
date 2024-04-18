use std::cmp::Ordering;
use std::fmt::Debug;
use serde::{Serialize, Deserialize, Serializer};
use std::rc::Rc;
use crate::compilation_unit::*;
use crate::util::{CharacterReader, count_first_whitespace_characters};

/// Represents a source location. This location includes
/// spanning lines and columns and the reference compilation unit.
#[derive(Clone, Deserialize)]
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

impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}:{}-{}:{}", self.first_line_number(), self.first_column() + 1, self.last_line_number(), self.last_column() + 1))
    }
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
        self.compilation_unit.get_column(self.first_offset)
    }

    /// Zero based last column of the location in code points.
    pub fn last_column(&self) -> usize {
        self.compilation_unit.get_column(self.last_offset)
    }

    pub fn character_count(&self) -> usize {
        self.compilation_unit.text()[self.first_offset..self.last_offset].chars().count()
    }

    /// Indicates whether a previous location and a next location
    /// have a line break in between.
    pub fn line_break(&self, other: &Self) -> bool {
        self.last_line_number() != other.first_line_number()
    }

    /// Returns the source text comprising the source location.
    pub fn text(&self) -> String {
        self.compilation_unit.text()[self.first_offset..self.last_offset].to_owned()
    }

    /// Shifts one character off this location until end-of-file.
    pub fn shift_until_eof(&self, count: usize) -> Location {
        let mut ch = CharacterReader::from(&self.compilation_unit.text()[self.first_offset..]);
        for _ in 0..count {
            if ch.next().is_none() {
                break;
            }
        }
        Self::with_offsets(&self.compilation_unit, self.first_offset + ch.index(), self.last_offset)
    }

    /// Shifts the count of whitespace characters in a text off this location.
    pub fn shift_whitespace(&self, text: &str) -> Location {
        self.shift_until_eof(count_first_whitespace_characters(text))
    }
}
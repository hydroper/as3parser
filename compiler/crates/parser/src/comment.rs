use crate::{Location, character_validation};

pub struct Comment {
    pub(crate) multiline: bool,
    pub(crate) content: String,
    pub(crate) location: Location,
}

impl Comment {
    pub fn new(multiline: bool, content: String, location: Location) -> Self {
        Self {
            multiline,
            content,
            location,
        }
    }

    pub fn multiline(&self) -> bool {
        self.multiline
    }

    /// The content of the comment. If it is a multi-line comment,
    /// it includes all the characters after `/*` until `*/` (exclusive);
    /// if it is a single-line comment, it includes all the characters after `//`
    /// until the next line terminator (exclusive) or end of program.
    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn location(&self) -> Location {
        self.location.clone()
    }

    pub fn set_location(&mut self, location: Location) {
        self.location = location;
    }

    /// Indicates whether the comment is an ASDoc comment preceding
    /// a specific item.
    pub fn is_asdoc(&self, item_location: &Location) -> bool {
        if self.multiline && self.content.starts_with('*') {
            let mut i: usize = self.location.last_offset;
            for (i_1, ch) in self.location.source().text[i..].char_indices() {
                i = i_1;
                if !(character_validation::is_whitespace(ch) || character_validation::is_line_terminator(ch)) {
                    break;
                }
            }
            item_location.first_offset == i
        } else {
            false
        }
    }
}
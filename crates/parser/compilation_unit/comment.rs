use crate::ns::*;

pub struct Comment {
    pub(crate) multiline: bool,
    pub(crate) content: RefCell<String>,
    pub(crate) location: RefCell<Location>,
}

impl Comment {
    pub fn new(multiline: bool, content: String, location: Location) -> Self {
        Self {
            multiline,
            content: RefCell::new(content),
            location: RefCell::new(location),
        }
    }

    pub fn multiline(&self) -> bool {
        self.multiline
    }

    /// The content of the comment.
    /// * If it is a multi-line comment, it includes all the characters after `/*` until `*/` (exclusive).
    /// * If it is a single-line comment, it includes all the characters after `//`
    /// until the next line terminator (exclusive) or end of program.
    pub fn content(&self) -> String {
        self.content.borrow().clone()
    }

    pub fn set_content(&self, content: String) {
        self.content.replace(content);
    }

    pub fn location(&self) -> Location {
        self.location.borrow().clone()
    }

    pub fn set_location(&self, location: Location) {
        self.location.replace(location);
    }

    /// Indicates whether the comment is an AsDoc comment preceding
    /// a specific location.
    pub fn is_asdoc(&self, location_to_precede: &Location) -> bool {
        if self.multiline && self.content.borrow().starts_with('*') {
            let mut i: usize = self.location.borrow().last_offset;
            for (i_1, ch) in self.location.borrow().compilation_unit().text()[i..].char_indices() {
                i = i_1;
                if !(CharacterValidator::is_whitespace(ch) || CharacterValidator::is_line_terminator(ch)) {
                    break;
                }
            }
            i += self.location.borrow().last_offset;
            location_to_precede.first_offset == i
        } else {
            false
        }
    }
}
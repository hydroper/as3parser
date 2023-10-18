use crate::Location;

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
}
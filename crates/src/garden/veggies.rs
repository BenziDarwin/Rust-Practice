// Accessibility modifiers are required in these kind of files.

pub struct Carrots {
    pub field: String,
    pub size: i32,
}

impl Carrots {
    pub fn new(field: String, size: i32) -> Self {
        Self {
            field: field,
            size: size,
        }
    }
}

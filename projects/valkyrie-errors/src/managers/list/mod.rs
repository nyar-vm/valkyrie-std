use super::*;

#[derive(Copy, Clone, Debug)]
pub struct TextSpan {
    pub file: FileID,
    pub head: usize,
    pub tail: usize,
}
impl TextSpan {
    pub fn as_label(&self, message: String) -> Label<(FileID, Range<usize>)> {
        Label::new((self.file, self.head..self.tail)).with_message(message)
    }
}

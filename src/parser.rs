#[derive(Default, Clone, Debug)]
pub struct Parser {
    buffer: Vec<u8>,
}

impl Parser {
    pub fn read_fix_message(&mut self) -> Result<Option<String>, ParserError> {
        todo!()
    }

    pub fn add_to_stream(&mut self, read: &[u8]) {
        self.buffer.extend_from_slice(read)
    }
}

#[derive(Debug)]
pub enum ParserError {}

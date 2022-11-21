#[derive(Default, Clone, Debug)]
pub struct Parser {
    buffer: Vec<u8>,
}

impl Parser {
    pub fn read_fix_message(&mut self) -> Result<Option<Vec<u8>>, ParserError> {
        Ok(read_fix(&mut self.buffer))
    }

    pub fn add_to_stream(&mut self, read: &[u8]) {
        self.buffer.extend_from_slice(read)
    }
}

trait Find<T> {
    fn find(&self, item: T) -> Option<usize>;
}

impl Find<&[u8]> for Vec<u8> {
    fn find(&self, item: &[u8]) -> Option<usize> {
        let mut index = 0;
        for _c in self {
            if item.len() >= self.len() - index {
                return None;
            }
            if &self[index..index + item.len()] == item {
                return Some(index);
            }
            index += 1;
        }
        None
    }
}
impl Find<char> for Vec<u8> {
    fn find(&self, item: char) -> Option<usize> {
        let mut index = 0;
        for c in self {
            if c == &(item as u8) {
                return Some(index);
            }
            index += 1;
        }
        None
    }
}
impl Find<&[u8]> for [u8] {
    fn find(&self, item: &[u8]) -> Option<usize> {
        let mut index = 0;
        for _c in self {
            if item.len() >= self.len() - index {
                return None;
            }
            if &self[index..index + item.len()] == item {
                return Some(index);
            }
            index += 1;
        }
        None
    }
}
impl Find<char> for [u8] {
    fn find(&self, item: char) -> Option<usize> {
        let mut index = 0;
        for c in self {
            if c == &(item as u8) {
                return Some(index);
            }
            index += 1;
        }
        None
    }
}

// void addToStream( const std::string& str )
// { m_buffer.append( str ); }

// bool Parser::readFixMessage( std::string& str )
// EXCEPT ( MessageParseError )
//     {
//     std::string::size_type pos = 0;

//     if( m_buffer.length() < 2 ) return false;
//     pos = m_buffer.find( "8=" );
//     if( pos == std::string::npos ) return false;
//     m_buffer.erase( 0, pos );

//     int length = 0;

//     try
//     {
//         if( extractLength(length, pos, m_buffer) )
//         {
//         pos += length;
//         if( m_buffer.size() < pos )
//             return false;

//         pos = m_buffer.find( "\00110=", pos-1 );
//         if( pos == std::string::npos ) return false;
//         pos += 4;
//         pos = m_buffer.find( "\001", pos );
//         if( pos == std::string::npos ) return false;
//         pos += 1;

//         str.assign( m_buffer, 0, pos );
//         m_buffer.erase( 0, pos );
//         return true;
//         }
//     }
//     catch( MessageParseError& e )
//     {
//         if( length > 0 )
//         m_buffer.erase( 0, pos + length );
//         else
//         m_buffer.erase();

//         throw e;
//     }

//     return false;
//     }
// }
/// Returns `Some<String>` if it can find a potential fix message otherwise returns `None`
/// Drains the buffer.
pub fn read_fix(buffer: &mut Vec<u8>) -> Option<Vec<u8>> {
    if buffer.len() < 2 {
        return None;
    }
    let pos: Option<usize> = buffer.find("8=".as_bytes());
    if pos == None || pos == Some(usize::MAX) {
        return None;
    }
    let pos = pos?;
    buffer.drain(..pos); //drain until 8=
    if let Some((len, mut pos)) = extract_length(&buffer) {
        pos += len;
        if buffer.len() < pos {
            return None;
        }
        let found = buffer[pos - 1..].find("\x0110=".as_bytes());
        pos = found? + pos - 1;
        // TODO should we return err if position of found is too large?
        pos += 4;
        let found = buffer[pos..].find('\x01');
        pos += found?;
        pos += 1;
        return Some(buffer.drain(..pos).collect());
    }
    None
}

/// Returns `Option<(pos, len)>` if it can find the length in the fix message otherwise returns `None`.
///
pub fn extract_length(buffer: &[u8]) -> Option<(usize, usize)> {
    let start = buffer.find("\x019=".as_bytes())? + 3;
    let end = buffer[start..].find('\x01')? + start;
    let str_len = &buffer[start..end];
    match std::str::from_utf8(str_len) {
        Ok(s) => {
            let out_len = s.parse::<usize>().ok()?;
            Some((end + 1, out_len))
        }
        Err(_) => None,
    }
}

pub fn read_version(buffer: &[u8]) -> Option<&str> {
    let pos: Option<usize> = buffer.find("8=".as_bytes());
    if pos == None || pos == Some(usize::MAX) {
        None
    } else {
        let pos = pos.unwrap();
        let found = buffer[pos..].find('\x01');
        let end = found.unwrap();
        match std::str::from_utf8(&buffer[pos + 2..end]) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum ParserError {}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    pub fn two_in_one() {
        let buffer = b"8=FIX.4.4\x019=57\x0135=A\x0134=1\x0149=ISLD\x0152=00000000-00:00:00\x0156=TW\x0198=0\x01108=30\x0110=0\x018=FIX.4.4\x019=45\x0135=5\x0134=2\x0149=ISLD\x0152=00000000-00:00:00\x0156=TW\x0110=0\x01";
        println!("{}", buffer.iter().map(|b| *b as char).collect::<String>());
        let mut parser = Parser::default();
        parser.add_to_stream(buffer);
        let msg = parser.read_fix_message();
        assert!(msg.is_ok());
        assert!(msg.unwrap().is_some());
        assert!(!parser.buffer.is_empty());
        println!(
            "{}",
            parser.buffer.iter().map(|b| *b as char).collect::<String>()
        );
        let msg = parser.read_fix_message();
        assert!(msg.is_ok());
        assert!(msg.unwrap().is_some());
    }
}

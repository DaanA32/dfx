#[macro_use]
extern crate afl;

use dfx_core::parser::Parser;

fn main() {
    let mut parser = Parser::default();
    fuzz!(|data: &[u8]| {
        parser.add_to_stream(data);
        parser.read_fix_message();
    });
}

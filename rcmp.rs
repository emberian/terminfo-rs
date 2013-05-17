extern mod terminfo;
use terminfo::*;
use std::os;

fn main() {
    match searcher::open(os::args()[1]) {
        Ok(reader) => parser::compiled::parse(reader, false),
        Err(s) => fail!(s)
    };
}

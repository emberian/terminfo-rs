/// compiled terminfo format parsing (term(5))

use core::io::Reader;
use terminfo::TermInfo;

static negone: i16 = 0xFFFFi16;

macro_rules! eofe(
    ($inp:ident) => (
        if $inp.len() != $inp_size {
            return Err(fmt!("invalid file: expected %d bytes in the $inp section but hit EOF",
                            $inp_size));
        }
    );
);

pub fn parse(file: Reader) -> Result<~TermInfo, &str> {
    // Check magic number
    let magic = file.read_byte();
    if (magic != 0x011A) {
        return Err(fmt!("invalid magic number. expected %x but found %x", 0x011A, magic));
    }

    // Get sizes
    let names_size   = file.read_le_i16();
    let bools_size   = file.read_le_i16();
    let numbers_size = file.read_le_i16();
    let offsets_size = file.read_le_i16();
    let strings_size = file.read_le_i16();

    // Extract names
    let names: ~[&str] = vec::with_capacity(3);
    let names_bytes = file.read_bytes(names_size);

    if names_bytes.len() != names_size {
        return Err(fmt!("invalid file: expected %d bytes in the names section but hit EOF",
                        names_size));
    }

    for str::from_bytes(names_bytes).each_split_char('|') |s| {
        names.push(s);
    }

    let bools: = file.read_bytes(bools_size);
    let numbers: = file.read_bytes(numbers_size);
    let offsets = file.read_bytes(offsets_size);
    let strings = file.read_bytes(strings_size);

    eofe!(bools);
    eofe!(numbers);
    eofe!(offsets);
    eofe!(strings);
}

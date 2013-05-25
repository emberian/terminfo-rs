extern mod terminfo;
use terminfo::*;
use std::os;
use std::str;

fn dump(ti: ~TermInfo) {
    for ti.names.each |&n| {
        println(fmt!("name: %s", n));
    }
    for ti.bools.each |&k, v| {
        println(fmt!("%s is set", k));
    }
    for ti.numbers.each |&k, &v| {
        println(fmt!("%s#%u", k, v as uint));
    }
    for ti.strings.each |&k, v| {
        if str::is_utf8(*v) {
            println(fmt!("%s=%?", k, str::from_bytes(*v)));
        } else {
            println(fmt!("%s=%?", k, v));
        }
    }
}

fn main() {
    match searcher::open(os::args()[1]) {
        Ok(reader) => match parser::compiled::parse(reader, false) {
            Ok(ti) => dump(ti),
            Err(s) => fail!(s)
        },
        Err(s) => fail!(s)
    };
}


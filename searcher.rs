/// Implement ncurses-compatible database discovery

use core::os::getenv;
use core::str;
use core::io::{file_reader, Reader};
use path = core::path::PosixPath;

/// Return path to database entry for `term`
pub fn get_dbpath_for_term(term: &str) -> Option<~path> {
    if term.len() == 0 {
        return None;
    }

    let mut dirs_to_search = ~[];
    let first_char = term.substr(0, 1);

    // Find search directory
    match getenv(~"TERMINFO") {
        Some(dir) => dirs_to_search.push(path(dir)),
        None => {
            match getenv(~"TERMINFO_DIRS") {
                Some(dirs) => for str::each_split_char(dirs, ':') |i| {
                    if i == ~"" {
                        dirs_to_search.push(path(~"/usr/share/terminfo"));
                    } else {
                        dirs_to_search.push(path(i.to_owned()));
                    }
                },
                // Found nothing, use the default path
                None => dirs_to_search.push(path(~"/usr/share/terminfo"))
            }
        }
    };

    // Look for the terminal in all of the search directories
    for dirs_to_search.each |p| {
        let newp = ~p.push_many(&[first_char.to_owned(), term.to_owned()]);
        if os::path_exists(p) && os::path_exists(newp) {
            return Some(newp);
        }
    }
    None
}

/// Return open file for `term`
fn open(term: &str) -> Result<@Reader, ~str> {
    match get_dbpath_for_term(term) {
        Some(x) => file_reader(x),
        None => Err(fmt!("could not find terminfo entry for %s", term))
    }
}

#[test]
fn test_get_dbpath_for_term() {
    // woefully inadequate test coverage
    use core::os::{setenv, unsetenv};
    fn x(t: &str) -> ~str { get_dbpath_for_term(t).expect("no terminfo entry found").to_str() };
    assert!(x("screen") == ~"/usr/share/terminfo/s/screen");
    assert!(get_dbpath_for_term("") == None);
    setenv("TERMINFO_DIRS", ":");
    assert!(x("screen") == ~"/usr/share/terminfo/s/screen");
    unsetenv("TERMINFO_DIRS");
}

#[test]
fn test_open() {
    open("screen");
    let t = open("nonexistent terminal that hopefully does not exist");
    assert!(t.is_err());
}

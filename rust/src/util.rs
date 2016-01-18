use regex::{Captures};
use std::str::FromStr;

/// Takes the first two capture groups from a regex result and turns them into a version
/// integer 2-tuple
pub fn map_first_captures(captures: Captures) -> (i8, i8) {
    let major_version = i8::from_str(&captures[1]).unwrap();
    let minor_version = i8::from_str(&captures[2]).unwrap();
    (major_version, minor_version)
}

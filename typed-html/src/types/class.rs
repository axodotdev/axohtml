use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use super::Id;

/// A valid CSS class.
///
/// A CSS class is a non-empty string that starts with an alphanumeric character
/// and is followed by any number of alphanumeric characters and the
/// `_`, `-` and `.` characters.
///
/// See also [`Id`].
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Class(String);

impl Class {
    /// Construct a new class name from a string.
    ///
    /// Panics if the provided string is invalid.
    pub fn new<S: Borrow<str>>(s: S) -> Self {
        let s = s.borrow();
        Self::from_str(s).unwrap_or_else(|err| {
            panic!(
                "axohtml::types::Class: {:?} is not a valid class name: {}",
                s, err
            )
        })
    }
}

// w3.org defines a strict grammar for classes in the CSS parser at
// https://www.w3.org/TR/CSS21/grammar.html#scanner, as shown below where
// ident is the class name:
//  ident		-?{nmstart}{nmchar}*
//  nmstart		[_a-z]|{nonascii}|{escape}
//  nmchar		[_a-z0-9-]|{nonascii}|{escape}
//  nonascii	[\240-\377] //Note: this is 0xA0 to 0xFF in hex
//  unicode		\\{h}{1,6}(\r\n|[ \t\r\n\f])?
//  escape		{unicode}|\\[^\r\n\f0-9a-f]
// w3.org defines a very different grammar for class names on the HTML side at
// https://www.w3.org/TR/2011/WD-html5-20110525/elements.html#classes, which
// has no limits other than spaces not being allowed. This implementation is
// for typed html and will use the HTML grammar.
impl FromStr for Class {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for c in s.chars() {
            if c.is_whitespace() {
                return Err("class name may not contain spaces");
            }
        }
        Ok(Class(s.to_string()))
    }
}

impl From<Id> for Class {
    fn from(id: Id) -> Self {
        Class(id.to_string())
    }
}

impl<'a> TryFrom<&'a str> for Class {
    type Error = &'static str;
    fn try_from(str: &'a str) -> Result<Self, Self::Error> {
        Class::from_str(str)
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Display::fmt(&self.0, f)
    }
}

impl Deref for Class {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test_from_string() {
    use crate::types::Class;
    // This test covers some tricky examples from TailwindsCSS
    // that don't quite pass the strict CSS-style grammar but
    // are valid in HTML.

    assert_eq!(
        Ok(Class(String::from("-bottom"))),
        Class::from_str("-bottom")
    );

    assert_eq!(
        Ok(Class(String::from("top-[3px]"))),
        Class::from_str("top-[3px]")
    );

    assert_eq!(
        Ok(Class(String::from("md:top-6"))),
        Class::from_str("md:top-6")
    );

    assert_eq!(
        Ok(Class(String::from("w-1/2"))),
        Class::from_str("w-1/2")
    );

    assert_eq!(
        Ok(Class(String::from("bg-[#50d71e]"))),
        Class::from_str("bg-[#50d71e]")
    );
}

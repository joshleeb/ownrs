use crate::{
    is_whitespace,
    owner::{owner, Owner},
};
use nom::{char, named, tag, take_till1, types::CompleteStr, ws};
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum Directive {
    NoParent,
    StarGlob,
    Owner(Owner),
    FilePath(PathBuf),
}

impl Directive {
    fn file_path<'a>(path: CompleteStr<'a>) -> Self {
        Directive::FilePath((*path).into())
    }
}

named!(pub(crate) directive<CompleteStr, Directive>, ws!(alt!(
        char!('*') => { |_| Directive::StarGlob } |
        pair!(tag!("set"), tag!("noparent")) => { |_| Directive::NoParent } |
        preceded!(tag!("file:"), take_till1!(is_whitespace)) => { Directive::file_path } |
        owner => { Directive::Owner }
)));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star() {
        let (rem, parsed) = directive(CompleteStr("*")).unwrap();

        assert_eq!(parsed, Directive::StarGlob);
        assert!(rem.is_empty());
    }

    #[test]
    fn no_parent() {
        let (rem, parsed) = directive(CompleteStr("set noparent")).unwrap();

        assert_eq!(parsed, Directive::NoParent);
        assert!(rem.is_empty());
    }

    #[test]
    fn no_parent_ws() {
        let (rem, parsed) = directive(CompleteStr("set   noparent")).unwrap();

        assert_eq!(parsed, Directive::NoParent);
        assert!(rem.is_empty());
    }

    #[test]
    fn filepath_absolute() {
        let (rem, parsed) = directive(CompleteStr("file: /absolute/path")).unwrap();

        assert_eq!(parsed, Directive::FilePath("/absolute/path".into()));
        assert!(rem.is_empty());
    }

    #[test]
    fn filepath_relative() {
        let (rem, parsed) = directive(CompleteStr("file: ../relative/path")).unwrap();

        assert_eq!(parsed, Directive::FilePath("../relative/path".into()));
        assert!(rem.is_empty());
    }

    #[test]
    fn filepath_ws() {
        let (rem, parsed) = directive(CompleteStr("file:   /absolute/path")).unwrap();

        assert_eq!(parsed, Directive::FilePath("/absolute/path".into()));
        assert!(rem.is_empty());
    }

    #[test]
    fn owner() {
        let (rem, parsed) = directive(CompleteStr("owner")).unwrap();

        assert_eq!(parsed, Directive::Owner(Owner::Text("owner".into())));
        assert!(rem.is_empty());
    }
}

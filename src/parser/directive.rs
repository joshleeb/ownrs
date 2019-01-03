use crate::parser::{
    owner::{owner, Owner},
    ws_or_comment,
};
use nom::{
    alt_sep, call, char, eof, error_position, exact, named, pair_sep, preceded_sep, sep, tag,
    take_till1, terminated, tuple, tuple_parser, types::CompleteStr, wrap_sep, ws,
};
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub(crate) enum Directive {
    NoParent,
    StarGlob,
    Owner(Owner),
    FilePath(PathBuf),
}

named!(pub(crate) directive<CompleteStr, Directive>, ws!(alt!(
        char!('*') => {
            |_| Directive::StarGlob
        } |
        pair!(tag!("set"), tag!("noparent")) => {
            |_| Directive::NoParent
        } |
        preceded!(tag!("file:"), take_till1!(ws_or_comment)) => {
            |path: CompleteStr| Directive::FilePath((*path).into())
        } |
        owner => {
            |x: Owner| Directive::Owner(x)
        }
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

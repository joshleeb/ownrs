use crate::{
    error::NomResult,
    is_whitespace,
    owner::{owner, Owner},
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::{char, multispace0, multispace1},
    combinator::map,
    error::context,
    sequence::{pair, preceded, separated_pair},
};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Directive {
    NoParent,
    StarGlob,
    FilePath(PathBuf),
    Owner(Owner),
}

pub(crate) fn directive(input: &str) -> NomResult<Directive> {
    let star_glob = char('*');
    let no_parent = separated_pair(tag("set"), multispace1, tag("noparent"));
    let file_path = preceded(pair(tag("file:"), multispace0), take_till1(is_whitespace));

    context(
        "directive",
        alt((
            map(star_glob, |_| Directive::StarGlob),
            map(no_parent, |_| Directive::NoParent),
            map(file_path, |path: &str| Directive::FilePath(path.into())),
            map(owner, Directive::Owner),
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star() {
        let (rem, parsed) = directive("*").unwrap();

        assert_eq!(parsed, Directive::StarGlob);
        assert!(rem.is_empty());
    }

    #[test]
    fn no_parent() {
        let (rem, parsed) = directive("set noparent").unwrap();

        assert_eq!(parsed, Directive::NoParent);
        assert!(rem.is_empty());
    }

    #[test]
    fn no_parent_ws() {
        let (rem, parsed) = directive("set   noparent").unwrap();

        assert_eq!(parsed, Directive::NoParent);
        assert!(rem.is_empty());
    }

    #[test]
    fn filepath_absolute() {
        let (rem, parsed) = directive("file: /absolute/path").unwrap();

        assert_eq!(parsed, Directive::FilePath("/absolute/path".into()));
        assert!(rem.is_empty());
    }

    #[test]
    fn filepath_relative() {
        let (rem, parsed) = directive("file: ../relative/path").unwrap();

        assert_eq!(parsed, Directive::FilePath("../relative/path".into()));
        assert!(rem.is_empty());
    }

    #[test]
    fn filepath_ws() {
        let (rem, parsed) = directive("file:   /absolute/path").unwrap();

        assert_eq!(parsed, Directive::FilePath("/absolute/path".into()));
        assert!(rem.is_empty());
    }

    #[test]
    fn owner() {
        let (rem, parsed) = directive("owner").unwrap();

        assert_eq!(parsed, Directive::Owner(Owner::Text("owner".into())));
        assert!(rem.is_empty());
    }
}

use crate::{
    directive::{directive, Directive},
    error::NomResult,
};
use globset::{Error, Glob};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, multispace0, multispace1},
    combinator::map_res,
    error::context,
    sequence::{preceded, terminated, tuple},
};

#[derive(Debug, PartialEq)]
pub struct PerFile {
    pub glob: Glob,
    pub directive: Directive,
}

pub(crate) fn per_file(input: &str) -> NomResult<PerFile> {
    let (rem, parsed) = context(
        "per-file",
        tuple((
            terminated(tag("per-file"), multispace1),
            map_res(take_until("="), str_to_glob),
            preceded(terminated(char('='), multispace0), multispace0),
            directive,
        )),
    )(input)?;

    Ok((
        rem,
        PerFile {
            glob: parsed.1,
            directive: parsed.3,
        },
    ))
}

fn str_to_glob(s: &str) -> Result<Glob, Error> {
    let mut glob_str = s.trim().to_string();

    // Required to be compatable with the globs used in OWNERS files.
    if !glob_str.starts_with("*") {
        glob_str = format!("*{}", glob_str);
    }

    Glob::new(&glob_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_per_file(glob_str: &str, directive: Directive) -> PerFile {
        PerFile {
            glob: Glob::new(glob_str).unwrap(),
            directive,
        }
    }

    #[test]
    fn glob_star_directive() {
        let (rem, parsed) = per_file("per-file *.rs = *").unwrap();

        assert_eq!(parsed, create_per_file("*.rs", Directive::StarGlob));
        assert!(rem.is_empty());
    }

    #[test]
    fn compatability_glob() {
        let (rem, parsed) = per_file("per-file Cargo.toml = *").unwrap();

        assert_eq!(parsed, create_per_file("*Cargo.toml", Directive::StarGlob));
        assert!(rem.is_empty());
    }

    #[test]
    fn glob_ws_extra() {
        let (rem, parsed) = per_file("per-file   *.rs   =   *").unwrap();

        assert_eq!(parsed, create_per_file("*.rs", Directive::StarGlob));
        assert!(rem.is_empty());
    }

    #[test]
    fn glob_ws_reduced() {
        let (rem, parsed) = per_file("per-file *.rs=*").unwrap();

        assert_eq!(parsed, create_per_file("*.rs", Directive::StarGlob));
        assert!(rem.is_empty());
    }

    #[test]
    fn invalid_glob() {
        assert!(per_file("per-file invalid-{glob = owner").is_err());
    }
}

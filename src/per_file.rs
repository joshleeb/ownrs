use crate::directive::{directive, Directive};
use globset::{Error, Glob};
use nom::{char, map_res, named, tag, take_until1, types::CompleteStr, ws};

#[derive(Debug, PartialEq)]
pub struct PerFile {
    pub glob: Glob,
    pub directive: Directive,
}

fn str_to_glob(s: &str) -> Result<Glob, Error> {
    let mut glob_str = s.trim().to_string();

    // Required to be compatable with the globs used in OWNERS files.
    if !glob_str.starts_with("*") {
        glob_str = format!("*{}", glob_str);
    }

    Glob::new(&glob_str)
}

named!(pub(crate) per_file<CompleteStr, PerFile>, ws!(do_parse!(
    tag!("per-file") >>
    glob: map_res!(take_until1!("="), |ref s: CompleteStr| str_to_glob(s)) >>
    char!('=') >>
    directive: directive >>
    (PerFile{glob, directive})
)));

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
        let (rem, parsed) = per_file(CompleteStr("per-file *.rs = *")).unwrap();

        assert_eq!(parsed, create_per_file("*.rs", Directive::StarGlob));
        assert!(rem.is_empty());
    }

    #[test]
    fn compatability_glob() {
        let (rem, parsed) = per_file(CompleteStr("per-file Cargo.toml = *")).unwrap();

        assert_eq!(parsed, create_per_file("*Cargo.toml", Directive::StarGlob));
        assert!(rem.is_empty());
    }

    #[test]
    fn glob_ws_extra() {
        let (rem, parsed) = per_file(CompleteStr("per-file   *.rs   =   *")).unwrap();

        assert_eq!(parsed, create_per_file("*.rs", Directive::StarGlob));
        assert!(rem.is_empty());
    }

    #[test]
    fn glob_ws_reduced() {
        let (rem, parsed) = per_file(CompleteStr("per-file*.rs=*")).unwrap();

        assert_eq!(parsed, create_per_file("*.rs", Directive::StarGlob));
        assert!(rem.is_empty());
    }

    #[test]
    fn invalid_glob() {
        assert!(per_file(CompleteStr("per-file invalid-{glob = owner")).is_err());
    }
}

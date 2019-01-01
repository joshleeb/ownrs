use crate::parser::directive::{directive, Directive};
use globset::{Error, Glob};
use nom::{
    call, char, do_parse_sep, eof, error_position, exact, map_res, named, sep, tag, take_until1,
    terminated, tuple, tuple_parser, types::CompleteStr, wrap_sep, ws,
};

#[derive(Debug, PartialEq)]
pub(crate) struct PerFile {
    glob: Glob,
    directive: Directive,
}

fn str_to_glob(s: &str) -> Result<Glob, Error> {
    let mut glob_str = s.trim().to_string();
    // Required to be compatable with the globs used inO WNERS files.
    if !glob_str.starts_with("*") {
        glob_str = format!("*{}", glob_str);
    }
    Glob::new(&glob_str)
}

named!(pub(crate) per_file<CompleteStr, PerFile>, exact!(ws!(do_parse!(
    tag!("per-file") >>
    glob: map_res!(take_until1!("="), |ref s: CompleteStr| str_to_glob(s)) >>
    char!('=') >>
    directive: directive >>
    (PerFile{glob, directive})
))));

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
    fn invalid_directive() {
        assert!(per_file(CompleteStr("perfile *.rs = owner")).is_err());
        assert!(per_file(CompleteStr("per-file *.rs owner")).is_err());
        assert!(per_file(CompleteStr("per-file *.rs = owner invalid")).is_err());
        assert!(per_file(CompleteStr("per-file owner")).is_err());
        assert!(per_file(CompleteStr("per-file *.rs = per-file *.rs = owner")).is_err());
    }

    #[test]
    fn invalid_glob() {
        assert!(per_file(CompleteStr("per-file invalid-{glob = owner")).is_err());
    }
}

use crate::parser::{
    directive::{directive, Directive},
    per_file::{per_file, PerFile},
};
use nom::{
    alt, call, eof, error_position, exact, named, terminated, tuple, tuple_parser,
    types::CompleteStr,
};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Directive(Directive),
    PerFile(PerFile),
}

named!(pub(crate) statement<CompleteStr, Statement>, exact!(alt!(
        per_file => {
            |x| Statement::PerFile(x)
        } |
        directive => {
            |x| Statement::Directive(x)
        }
)));

#[cfg(test)]
mod tests {
    use super::*;
    use globset::Glob;

    #[test]
    fn directive() {
        let (rem, parsed) = statement(CompleteStr("*")).unwrap();

        assert_eq!(parsed, Statement::Directive(Directive::StarGlob));
        assert!(rem.is_empty());
    }

    #[test]
    fn per_file() {
        let (rem, parsed) = statement(CompleteStr("per-file *.rs = *")).unwrap();

        assert_eq!(
            parsed,
            Statement::PerFile(PerFile {
                glob: Glob::new("*.rs").unwrap(),
                directive: Directive::StarGlob
            })
        );
        assert!(rem.is_empty());
    }
}

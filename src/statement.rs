use crate::{
    directive::{directive, Directive},
    error::NomResult,
    per_file::{per_file, PerFile},
};
use nom::{branch::alt, combinator::map, error::context};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Directive(Directive),
    PerFile(PerFile),
}

pub(crate) fn statement(input: &str) -> NomResult<Statement> {
    context(
        "statement",
        alt((
            map(per_file, Statement::PerFile),
            map(directive, Statement::Directive),
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use globset::Glob;

    #[test]
    fn directive() {
        let (rem, parsed) = statement("*").unwrap();

        assert_eq!(parsed, Statement::Directive(Directive::StarGlob));
        assert!(rem.is_empty());
    }

    #[test]
    fn per_file() {
        let (rem, parsed) = statement("per-file *.rs = *").unwrap();

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

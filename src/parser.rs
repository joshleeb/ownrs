pub(crate) use self::{
    directive::Directive, owner::Owner, per_file::PerFile, statement::Statement,
};

use self::{error::ParseError, statement::statement};
use nom::types::CompleteStr;

mod directive;
mod error;
mod owner;
mod per_file;
mod statement;

fn ws_or_comment(c: char) -> bool {
    c == ' ' || c == '#' || c == '\t' || c == '\n'
}

// TODO: I imagine it is more efficient to use nom to ignore inline comments rather than splitting
// the string. We should benchmark this and update the parser if that is the case.
fn remove_inline_comments(line: &str) -> &str {
    line.trim().split("#").nth(0).unwrap()
}

pub(crate) fn parse(input: &str) -> Result<Vec<Statement>, ParseError> {
    let filtered = input
        .split("\n")
        // Attach line numbers to each line.
        .enumerate()
        // Remove any inline comments from each line.
        .map(|(line_num, content)| (line_num + 1, remove_inline_comments(content)))
        // Ignore empty lines and comments.
        .filter(|(_, content)| !content.is_empty() && !content.starts_with("#"));

    let mut parsed = vec![];
    for (line_num, content) in filtered {
        statement(CompleteStr(content))
            .map(|(_, output)| parsed.push(output))
            .map_err(|e| ParseError::from_nom(line_num, e))?;
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(parse("").unwrap().is_empty());
    }

    #[test]
    fn empty_ws() {
        assert!(parse(" ").unwrap().is_empty());
    }

    #[test]
    fn comment() {
        assert!(parse("# comment").unwrap().is_empty());
    }

    #[test]
    fn comment_ws() {
        assert!(parse("#comment").unwrap().is_empty());
    }

    #[test]
    fn inline_comment() {
        assert_eq!(
            parse("set noparent # comment").unwrap(),
            vec![Statement::Directive(Directive::NoParent)]
        );
    }

    #[test]
    fn inline_comment_ws() {
        assert_eq!(
            parse("set noparent# comment").unwrap(),
            vec![Statement::Directive(Directive::NoParent)]
        );
    }

    #[test]
    fn multiline() {
        assert_eq!(
            parse("set noparent\n*").unwrap(),
            vec![
                Statement::Directive(Directive::NoParent),
                Statement::Directive(Directive::StarGlob)
            ]
        );
    }

    #[test]
    fn multiline_trailing_newline() {
        assert_eq!(
            parse("set noparent\n*\n").unwrap(),
            vec![
                Statement::Directive(Directive::NoParent),
                Statement::Directive(Directive::StarGlob)
            ]
        );
    }

    #[test]
    fn multiline_leading_newline() {
        assert_eq!(
            parse("\nset noparent\n*").unwrap(),
            vec![
                Statement::Directive(Directive::NoParent),
                Statement::Directive(Directive::StarGlob)
            ]
        );
    }

    #[test]
    fn multiline_comments() {
        assert_eq!(
            parse("set noparent # comment\n* # comment").unwrap(),
            vec![
                Statement::Directive(Directive::NoParent),
                Statement::Directive(Directive::StarGlob)
            ]
        );
    }
}

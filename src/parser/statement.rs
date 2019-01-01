use crate::parser::directive::Directive;

#[derive(Debug, PartialEq)]
pub(crate) enum Statement {
    Directive(Directive),
}

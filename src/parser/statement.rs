use crate::parser::{directive::Directive, per_file::PerFile};

#[derive(Debug, PartialEq)]
pub(crate) enum Statement {
    Directive(Directive),
    PerFile(PerFile),
}

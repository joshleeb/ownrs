use crate::parser::ws_or_comment;
use nom::{
    call, eof, error_position, exact, map, named, take_till1, terminated, tuple, tuple_parser,
    types::CompleteStr,
};

#[derive(Debug, PartialEq)]
pub(crate) enum Owner {
    Email(String),
    Handle(String),
    Text(String),
}

impl<'a> From<CompleteStr<'a>> for Owner {
    fn from(input: CompleteStr<'a>) -> Self {
        if input.starts_with("@") {
            return Owner::Handle(input.trim_start_matches("@").to_string());
        }
        if input.contains("@") && !input.ends_with("@") {
            return Owner::Email(input.to_string());
        }
        Owner::Text(input.to_string())
    }
}

named!(pub(crate) owner<CompleteStr, Owner>, map!(
    take_till1!(ws_or_comment), Owner::from)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle() {
        let (rem, parsed) = owner(CompleteStr("@handle")).unwrap();

        assert_eq!(parsed, Owner::Handle("handle".into()));
        assert!(rem.is_empty());
    }

    #[test]
    fn email() {
        let (rem, parsed) = owner(CompleteStr("name@domain")).unwrap();

        assert_eq!(parsed, Owner::Email("name@domain".into()));
        assert!(rem.is_empty());
    }

    #[test]
    fn text() {
        let (rem, parsed) = owner(CompleteStr("text")).unwrap();

        assert_eq!(parsed, Owner::Text("text".into()));
        assert!(rem.is_empty());
    }
}

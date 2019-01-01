use crate::parser::ws_or_comment;
use nom::{
    call, eof, error_position, exact, map_res, named, take_till1, terminated, tuple, tuple_parser,
    types::CompleteStr,
};

#[derive(Debug, PartialEq)]
pub(crate) enum Owner {
    Email(String),
    Handle(String),
    Text(String),
}

impl From<&str> for Owner {
    fn from(input: &str) -> Self {
        if input.starts_with("@") {
            return Owner::Handle(input.trim_start_matches("@").to_string());
        }
        if input.contains("@") {
            return Owner::Email(input.to_string());
        }
        Owner::Text(input.to_string())
    }
}

fn str_to_owner(input: CompleteStr) -> Result<Owner, ()> {
    Ok(Owner::from(input.to_string().as_ref()))
}

named!(pub(crate) owner<CompleteStr, Owner>, exact!(
    map_res!(take_till1!(ws_or_comment), str_to_owner)
));

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
    fn handle_invalid() {
        assert!(owner(CompleteStr("@ handle")).is_err());
        assert!(owner(CompleteStr("@h andle")).is_err());
    }

    #[test]
    fn email() {
        let (rem, parsed) = owner(CompleteStr("name@domain")).unwrap();

        assert_eq!(parsed, Owner::Email("name@domain".into()));
        assert!(rem.is_empty());
    }

    #[test]
    fn email_invalid() {
        assert!(owner(CompleteStr("name @domain")).is_err());
        assert!(owner(CompleteStr("name@ domain")).is_err());
        assert!(owner(CompleteStr("n ame@domain")).is_err());
        assert!(owner(CompleteStr("name@d omain")).is_err());
    }

    #[test]
    fn text() {
        let (rem, parsed) = owner(CompleteStr("text")).unwrap();

        assert_eq!(parsed, Owner::Text("text".into()));
        assert!(rem.is_empty());
    }

    #[test]
    fn text_invalid() {
        assert!(owner(CompleteStr("t ext")).is_err());
    }
}

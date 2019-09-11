use crate::is_whitespace;
use nom::{map, named, take_till1, types::CompleteStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Owner {
    Email(String),
    Handle(String),
    Text(String),
}

impl Owner {
    fn handle<'a>(handle: CompleteStr<'a>) -> Self {
        Owner::Handle(handle.trim_start_matches("@").to_string())
    }

    fn email<'a>(email: CompleteStr<'a>) -> Self {
        Owner::Email(email.to_string())
    }

    fn text<'a>(text: CompleteStr<'a>) -> Self {
        Owner::Text(text.to_string())
    }
}

impl<'a> From<CompleteStr<'a>> for Owner {
    fn from(input: CompleteStr<'a>) -> Self {
        if input.starts_with("@") {
            return Owner::handle(input);
        }
        if input.contains("@") && !input.ends_with("@") {
            return Owner::email(input);
        }
        Owner::text(input)
    }
}

named!(pub(crate) owner<CompleteStr, Owner>, map!(
    take_till1!(is_whitespace), Owner::from)
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

use crate::{error::NomResult, is_whitespace};
use nom::{bytes::complete::take_till1, combinator::map};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Owner {
    Email(String),
    Handle(String),
    Text(String),
}

impl Owner {
    fn handle<'a>(handle: &'a str) -> Self {
        Owner::Handle(handle.trim_start_matches("@").to_string())
    }

    fn email<'a>(email: &'a str) -> Self {
        Owner::Email(email.to_string())
    }

    fn text<'a>(text: &'a str) -> Self {
        Owner::Text(text.to_string())
    }
}

impl<'a> From<&'a str> for Owner {
    fn from(input: &'a str) -> Self {
        if input.starts_with("@") {
            return Owner::handle(input);
        }
        if input.contains("@") && !input.ends_with("@") {
            return Owner::email(input);
        }
        Owner::text(input)
    }
}

pub(crate) fn owner(input: &str) -> NomResult<Owner> {
    map(take_till1(is_whitespace), Owner::from)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle() {
        let (rem, parsed) = owner("@handle").unwrap();

        assert_eq!(parsed, Owner::Handle("handle".into()));
        assert!(rem.is_empty());
    }

    #[test]
    fn email() {
        let (rem, parsed) = owner("name@domain").unwrap();

        assert_eq!(parsed, Owner::Email("name@domain".into()));
        assert!(rem.is_empty());
    }

    #[test]
    fn text() {
        let (rem, parsed) = owner("text").unwrap();

        assert_eq!(parsed, Owner::Text("text".into()));
        assert!(rem.is_empty());
    }
}

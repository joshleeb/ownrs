use std::{error::Error, fmt, io};

pub(crate) type NomError = nom::Err<()>;
pub(crate) type NomResult<'a, T> = Result<(&'a str, T), NomError>;

#[derive(Debug)]
pub enum ParseError {
    Generic,
    FileNotFound,
    Syntax { line_num: usize, message: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Syntax { line_num, message } => {
                write!(f, "{} on line {}: {}", self.as_str(), line_num, message)
            }
            _ => write!(f, "{}", self.as_str()),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        self.as_str()
    }
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::NotFound => ParseError::FileNotFound,
            _ => ParseError::Generic,
        }
    }
}

impl ParseError {
    pub(crate) fn from_nom(line_num: usize, err: NomError) -> ParseError {
        ParseError::Syntax {
            line_num,
            message: err.to_string(),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            ParseError::Generic => "parser error",
            ParseError::FileNotFound => "no such file or directory",
            ParseError::Syntax { .. } => "syntax error",
        }
    }
}

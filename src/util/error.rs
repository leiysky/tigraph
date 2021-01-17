use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub msg: String,
    pub kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

impl From<mysql::Error> for Error {
    fn from(err: mysql::Error) -> Error {
        Error {
            msg: err.to_string(),
            kind: ErrorKind::Mysql,
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub enum ErrorKind {
    Parse,
    Internal,
    Mysql,
    Unknown,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Parse => write!(f, "ParseError"),
            ErrorKind::Internal => write!(f, "InternalError"),
            ErrorKind::Mysql => write!(f, "MysqlError"),
            ErrorKind::Unknown => write!(f, "UnknownError"),
        }
    }
}

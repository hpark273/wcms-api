use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum UserError {
    InvalidEmail(String),
    SameEmail,
    AlreadyActive,
    AlreadyInactive,
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            UserError::InvalidEmail(e) => write!(f, "invalid email: {e}"),
            UserError::SameEmail => write!(f, "email is the same as the current one"),
            UserError::AlreadyActive => write!(f, "user is already active"),
            UserError::AlreadyInactive => write!(f, "user is already inactive"),
        }
    }
}

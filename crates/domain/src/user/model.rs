use jiff::Timestamp;

use crate::user::error::UserError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub i32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(value: impl Into<String>) -> Result<Self, UserError> {
        let value = value.into();
        if !Self::is_valid(&value) {
            return Err(UserError::InvalidEmail(value));
        }
        Ok(Self(value.to_lowercase()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn is_valid(email: &str) -> bool {
        let parts: Vec<&str> = email.split('@').collect();
        matches!(parts.as_slice(), [local, domain] if !local.is_empty() && domain.contains('.'))
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password_hash: String,
    pub is_active: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl User {
    pub fn new(
        id: i32,
        email: impl Into<String>,
        password_hash: impl Into<String>,
    ) -> Result<Self, UserError> {
        let now = Timestamp::now();
        Ok(Self {
            id: UserId(id),
            email: Email::new(email.into())?,
            password_hash: password_hash.into(),
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn active(&mut self) -> Result<(), UserError> {
        if self.is_active {
            return Err(UserError::AlreadyActive);
        }
        self.is_active = true;
        self.touch();
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), UserError> {
        if !self.is_active {
            return Err(UserError::AlreadyInactive);
        }
        self.is_active = false;
        self.touch();
        Ok(())
    }

    pub fn change_email(&mut self, new_email: impl Into<String>) -> Result<(), UserError> {
        let new_email = Email::new(new_email.into())?;
        if self.email == new_email {
            return Err(UserError::SameEmail);
        }

        self.email = new_email;
        self.touch();
        Ok(())
    }

    pub fn change_password(&mut self, new_password_hash: impl Into<String>) {
        self.password_hash = new_password_hash.into();
        self.touch();
    }

    fn touch(&mut self) {
        self.updated_at = Timestamp::now();
    }
}

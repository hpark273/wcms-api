use jiff::Timestamp;

use crate::user::error::UserError;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
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
        let email = email.into();

        if !Self::is_valid_email(&email) {
            return Err(UserError::InvalidEmail(email));
        }

        let now = Timestamp::now();

        Ok(Self {
            id,
            email,
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
        let new_email = new_email.into();

        if !Self::is_valid_email(&new_email) {
            return Err(UserError::InvalidEmail(new_email));
        }
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

    fn is_valid_email(email: &str) -> bool {
        let parts: Vec<&str> = email.split('@').collect();
        matches!(parts.as_slice(), [local, domain] if !local.is_empty() && domain.contains('.'))
    }
}

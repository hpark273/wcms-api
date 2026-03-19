use async_trait::async_trait;

use crate::user::{CreateUserCommand, User, UserError, model::UserId};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user(&self, id: UserId) -> Result<User, UserError>;
    async fn create_user(&self, cmd: CreateUserCommand) -> Result<User, UserError>;
}

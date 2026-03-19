use async_trait::async_trait;

use crate::user::{User, UserError, model::UserId};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, UserError>;
    async fn save(&self, user: &User) -> Result<(), UserError>;
}

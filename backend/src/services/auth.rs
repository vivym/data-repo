use std::collections::HashSet;

use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
use deadpool_diesel::postgres::Pool;
use password_auth::verify_password;
use serde::Deserialize;

use crate::{
    domain::models::{user::UserModel, permission::PermissionModel},
    infra::repositories::{self, error::RepoError},
};

impl AuthUser for UserModel {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.hashed_password.as_bytes()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct Backend {
    db: Pool,
}

impl Backend {
    pub fn new(db: Pool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = UserModel;
    type Credentials = Credentials;
    type Error = RepoError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<UserModel> = repositories::user::try_get_by_username(
            &self.db, creds.username.clone(),
        ).await?;

        Ok(user.filter(|user| {
            verify_password(creds.password, &user.hashed_password)
                .ok()
                .is_some()
        }))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        repositories::user::try_get_by_id(&self.db, *user_id).await
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Permission {
    pub name: String,
}

impl From<&str> for Permission {
    fn from(name: &str) -> Self {
        Permission {
            name: name.to_string(),
        }
    }
}

impl Into<Permission> for PermissionModel {
    fn into(self) -> Permission {
        Permission { name: self.name }
    }
}

#[async_trait]
impl AuthzBackend for Backend {
    type Permission = Permission;

    async fn get_group_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        let perms = repositories::user::get_permissions(
            &self.db, user.id
        )
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(perms)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;

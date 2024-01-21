use axum::{
    extract::{State, Request},
    http::header,
    middleware::Next,
    response::{IntoResponse, Response},
};

use axum_extra::extract::cookie::CookieJar;
use futures_util::future::BoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use tower::{Layer, Service};
use std::task::{Context, Poll};

use crate::{
    infra::repositories,
    routes::auth::{
        error::AuthError,
        login::TokenClaims,
    },
    server::AppState, domain::models::user::UserModel,
};

pub async fn auth<B>(
    cookie_jar: CookieJar,
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AuthError> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = token.ok_or(AuthError::Unauthorized)?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(state.jwt_secret.as_ref()),
        &Validation::default(),
    )
        .map_err(|_| AuthError::InvalidToken)?
        .claims;

    let user = repositories::user::get_by_id(
        &state.pg_pool, claims.sub
    )
        .await
        .map_err(AuthError::RepoError)?;

    if !user.is_active {
        return Err(AuthError::UserNotActive);
    }

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

#[derive(Clone)]
pub struct AuthLayer {
    state: AppState,
    permission: Option<String>,
}

impl AuthLayer {
    pub fn new(state: AppState, permission: Option<String>) -> Self {
        Self {
            state,
            permission,
        }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthService {
            inner,
            state: self.state.clone(),
            permission: self.permission.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthService<S> {
    inner: S,
    state: AppState,
    permission: Option<String>,
}

impl<S> Service<Request> for AuthService<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let token = CookieJar::from_headers(&req.headers())
            .get("token")
            .map(|cookie| cookie.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(header::AUTHORIZATION)
                    .and_then(|auth_header| auth_header.to_str().ok())
                    .and_then(|auth_value| {
                        if auth_value.starts_with("Bearer ") {
                            Some(auth_value[7..].to_owned())
                        } else {
                            None
                        }
                    })
            });

        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);
        let jwt_secret = self.state.jwt_secret.clone();
        let pg_pool = self.state.pg_pool.clone();
        let permission = self.permission.clone();

        Box::pin(async move {
            let user: Result<UserModel, AuthError> = async move {
                let token = token.ok_or(AuthError::Unauthorized)?;

                let claims = decode::<TokenClaims>(
                    &token,
                    &DecodingKey::from_secret(jwt_secret.as_ref()),
                    &Validation::default(),
                )
                    .map_err(|_| AuthError::InvalidToken)?
                    .claims;

                let user = repositories::user::get_by_id(
                    &pg_pool, claims.sub
                )
                    .await
                    .map_err(AuthError::RepoError)?;

                if !user.is_active {
                    return Err(AuthError::UserNotActive);
                }

                if let Some(permission) = permission {
                    let perms = repositories::user::get_permissions(
                        &pg_pool, user.id
                    )
                        .await
                        .map_err(AuthError::RepoError)?;

                    if !perms.iter().any(|perm| perm.name == permission) {
                        return Err(AuthError::PermissionDenied);
                    }
                }

                Ok(user)
            }
                .await;

            match user {
                Ok(user) => {
                    req.extensions_mut().insert(user);
                    inner.call(req).await
                }
                Err(err) => return Ok(err.into_response()),
            }
        })
    }
}

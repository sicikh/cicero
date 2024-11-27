use async_trait::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use loco_rs::prelude::auth::{JWTWithUser, JWT};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MaybeJwt(pub Option<JWT>);

#[async_trait]
impl<S> FromRequestParts<S> for MaybeJwt
where
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Error> {
        match JWT::from_request_parts(parts, state).await {
            Ok(jwt) => Ok(Self(Some(jwt))),
            Err(Error::Unauthorized(_)) => Ok(Self(None)),
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MaybeJwtWithUser<T: Authenticable>(pub Option<JWTWithUser<T>>);

#[async_trait]
impl<S, T> FromRequestParts<S> for MaybeJwtWithUser<T>
where
    AppContext: FromRef<S>,
    S: Send + Sync,
    T: Authenticable,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Error> {
        match JWTWithUser::<T>::from_request_parts(parts, state).await {
            Ok(jwt) => Ok(Self(Some(jwt))),
            Err(Error::Unauthorized(_)) => Ok(Self(None)),
            Err(err) => Err(err),
        }
    }
}

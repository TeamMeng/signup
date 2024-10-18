mod auth;
mod detail;
mod login;
mod new;
mod password_helper;

use axum::{
    extract::{FromRequestParts, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub use auth::{decode_token, generate_jwt, is_valid, Claims};
pub use detail::ProfileDetails;
pub use login::Login;
pub use new::NewCustomer;
pub use password_helper::hash_password;
use tracing::warn;

use crate::Db;

// member_id SERIAL PRIMARY KEY,
// name VARCHAR NOT NULL,
// password VARCHAR NOT NULL,
// email VARCHAR NOT NULL UNIQUE
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Customer {
    pub member_id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
}

impl Customer {
    pub fn to_response(&self) -> ProfileDetails {
        ProfileDetails {
            name: self.name.clone(),
            email: self.email.clone(),
        }
    }
}

pub async fn verify_customer(State(db): State<Db>, req: Request, next: Next) -> Response {
    let (mut parts, body) = req.into_parts();
    let token =
        match TypedHeader::<Authorization<Bearer>>::from_request_parts(&mut parts, &db).await {
            Ok(TypedHeader(Authorization(bearer))) => bearer.token().to_string(),
            Err(e) => {
                let msg = format!("parse Authorization header failed: {}", e);
                warn!(msg);
                return (StatusCode::UNAUTHORIZED, msg).into_response();
            }
        };
    let req = match decode_token(&token, "secret") {
        Ok(token) => {
            if is_valid(&token.claims) {
                let mut req = Request::from_parts(parts, body);
                match db.get_by_member_id(token.claims.sub).await {
                    Ok(Some(cus)) => {
                        req.extensions_mut().insert(cus);
                        req
                    }
                    _ => {
                        let msg = "customer is not found";
                        warn!(msg);
                        return (StatusCode::NOT_FOUND, msg).into_response();
                    }
                }
            } else {
                let msg = "token is invalid failed";
                warn!(msg);
                return (StatusCode::FORBIDDEN, msg).into_response();
            }
        }
        Err(e) => {
            let msg = format!("verify token failed: {:?}", e);
            warn!(msg);
            return (StatusCode::FORBIDDEN, msg).into_response();
        }
    };
    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{create_handler, login_handler};
    use anyhow::Result;
    use axum::{body::Body, middleware::from_fn_with_state, routing::get, Json, Router};
    use tower::ServiceExt;

    async fn handler(_req: Request) -> impl IntoResponse {
        (StatusCode::OK, "Ok")
    }

    #[tokio::test]
    async fn verify_customer_middleware_should_work() -> Result<()> {
        let name = "TeamMeng";
        let password = "hunter42";
        let email = "Meng@123.com";
        let new_customer = NewCustomer::new(name, password, email);
        let (_tdb, db) = Db::test_new().await;
        create_handler(State(db.clone()), Json(new_customer)).await?;

        let login = Login::new(email, password);
        let token = login_handler(State(db.clone()), Json(login))
            .await?
            .to_string();

        let app = Router::new()
            .route("/", get(handler))
            .layer(from_fn_with_state(db.clone(), verify_customer))
            .with_state(db);

        let req = Request::builder()
            .uri("/")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())?;
        let ret = app.clone().oneshot(req).await?;
        assert_eq!(ret.status(), StatusCode::OK);
        Ok(())
    }
}

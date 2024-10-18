use crate::{error::AppError, hash_password, model::generate_jwt, Db, Login};
use axum::{extract::State, Json};

pub async fn login_handler(
    State(db): State<Db>,
    Json(mut login): Json<Login>,
) -> Result<Json<String>, AppError> {
    let password = hash_password(login.password);
    login.password = password;
    let ret = db.login(login).await?;

    match ret {
        Some(cus) => {
            let token = generate_jwt(cus.member_id, "secret")?;
            Ok(Json(token))
        }
        None => Err(AppError::NotFound),
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_handler, model::decode_token, NewCustomer};

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn login_handler_should_work() -> Result<()> {
        let (_tdb, db) = Db::test_new().await;
        let name = "TeamMeng";
        let password = "hunter42";
        let email = "Meng@123.com";

        let new_customer = NewCustomer::new(name, password, email);
        create_handler(State(db.clone()), Json(new_customer)).await?;

        let login = Login::new(email, password);

        let ret = login_handler(State(db.clone()), Json(login)).await?;
        let token = ret.to_string();

        let ret = decode_token(&token, "secret")?;

        let cus = db
            .get_by_member_id(ret.claims.sub)
            .await?
            .expect("Should exist");

        assert_eq!(cus.name, name);
        assert_eq!(cus.email, email);

        Ok(())
    }
}

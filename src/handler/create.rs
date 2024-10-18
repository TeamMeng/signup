use crate::{error::AppError, hash_password, Db, NewCustomer};
use axum::{extract::State, http::StatusCode, Json};

pub async fn create_handler(
    State(db): State<Db>,
    Json(mut new_customer): Json<NewCustomer>,
) -> Result<StatusCode, AppError> {
    let password = hash_password(new_customer.password);
    new_customer.password = password;
    db.create_customer(new_customer).await?;
    Ok(StatusCode::OK)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn create_handler_should_work() -> Result<()> {
        let (_tdb, db) = Db::test_new().await;

        let name = "TeamMeng";
        let password = "hunter42";
        let email = "Meng@123.com";
        let new_customer = NewCustomer::new(name, password, email);

        let ret = create_handler(State(db.clone()), Json(new_customer)).await?;
        assert_eq!(ret, StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn create_handler_should_failed() -> Result<()> {
        let (_tdb, db) = Db::test_new().await;

        let name = "TeamMeng";
        let password = "hunter42";
        let email = "Meng@123.com";
        let new_customer = NewCustomer::new(name, password, email);

        let _ = create_handler(State(db.clone()), Json(new_customer.clone())).await?;
        let ret = create_handler(State(db.clone()), Json(new_customer)).await;

        assert!(ret.is_err());

        Ok(())
    }
}

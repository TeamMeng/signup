use super::Customer;
use crate::{error::AppError, Db};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct NewCustomer {
    pub name: String,
    pub password: String,
    pub email: String,
}

impl Db {
    pub async fn create_customer(&self, new_customer: NewCustomer) -> Result<Customer, AppError> {
        let cus = sqlx::query_as(
            r#"
            INSERT INTO customers (name, password, email) VALUES ($1, $2, $3) RETURNING member_id, name, password, email
            "#,
        )
        .bind(new_customer.name)
        .bind(new_customer.password)
        .bind(new_customer.email)
        .fetch_one(&self.pool)
        .await?;

        Ok(cus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    impl NewCustomer {
        pub fn new(
            name: impl Into<String>,
            password: impl Into<String>,
            email: impl Into<String>,
        ) -> Self {
            Self {
                name: name.into(),
                password: password.into(),
                email: email.into(),
            }
        }
    }

    #[tokio::test]
    async fn create_customer_should_work() -> Result<()> {
        let name = "TeamMeng";
        let password = "hunter42";
        let email = "Meng@123.com";
        let new_customer = NewCustomer::new(name, password, email);
        let (_tdb, db) = Db::test_new().await;
        let cus = db.create_customer(new_customer).await?;
        assert_eq!(cus.name, name);
        assert_eq!(cus.password, password);
        assert_eq!(cus.email, email);
        Ok(())
    }
}

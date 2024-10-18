use super::Customer;
use crate::{error::AppError, Db};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Db {
    pub async fn login(&self, login: Login) -> Result<Option<Customer>, AppError> {
        let cus = sqlx::query_as(
            r#"
            SELECT member_id, name, password, email FROM customers WHERE email = $1 and password = $2
            "#,
        )
        .bind(login.email)
        .bind(login.password)
        .fetch_optional(&self.pool)
        .await?;

        Ok(cus)
    }

    pub async fn get_by_member_id(&self, member_id: i32) -> Result<Option<Customer>, AppError> {
        let cus = sqlx::query_as(
            r#"
            SELECT member_id, name, password, email FROM customers WHERE member_id = $1
            "#,
        )
        .bind(member_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(cus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::new::NewCustomer;
    use anyhow::Result;

    impl Login {
        pub fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
            Self {
                email: email.into(),
                password: password.into(),
            }
        }
    }

    #[tokio::test]
    async fn login_should_work() -> Result<()> {
        let name = "TeamMeng";
        let password = "hunter42";
        let email = "Meng@123.com";

        let new_customer = NewCustomer::new(name, password, email);
        let (_tdb, db) = Db::test_new().await;
        db.create_customer(new_customer).await?;

        let login = Login::new(email, password);
        let cus = db.login(login).await?.unwrap();

        assert_eq!(cus.name, name);
        assert_eq!(cus.password, password);
        assert_eq!(cus.email, email);
        Ok(())
    }

    #[tokio::test]
    async fn member_id_should_work() -> Result<()> {
        let name = "TeamMeng";
        let password = "hunter42";
        let email = "Meng@123.com";

        let new_customer = NewCustomer::new(name, password, email);
        let (_tdb, db) = Db::test_new().await;
        db.create_customer(new_customer).await?;

        let cus = db.get_by_member_id(1).await?.unwrap();

        assert_eq!(cus.member_id, 1);
        assert_eq!(cus.name, name);
        assert_eq!(cus.password, password);
        assert_eq!(cus.email, email);

        Ok(())
    }
}

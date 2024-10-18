mod error;
mod handler;
mod model;

use anyhow::Result;
use dotenv::dotenv;
use sqlx::PgPool;
use sqlx_db_tester::TestPg;
use std::{env, path::Path};

pub use handler::{create_handler, login_handler, profile_handler};
pub use model::{hash_password, verify_customer, Claims, Login, NewCustomer};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

#[derive(Clone)]
pub struct Db {
    pool: PgPool,
}

impl Db {
    pub async fn try_new() -> Result<Self> {
        dotenv().ok();

        let url = env::var("DATABASE_URL")?;

        let pool = PgPool::connect(&url).await?;
        Ok(Self { pool })
    }

    pub async fn test_new() -> (TestPg, Self) {
        let tdb = TestPg::new(
            "postgres://postgres:postgres@localhost:5432".to_string(),
            Path::new("./migrations"),
        );
        let pool = tdb.get_pool().await;
        (tdb, Self { pool })
    }
}

use anyhow::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct Storage {
    db: Option<DatabaseConnection>,
}

pub async fn new() -> Result<Storage> {
    let database_url :&str ="mysql://root@test:123456@tcp(127.0.0.1:2881)/test?charset=utf8mb4&parseTime=True&loc=Local";

    println!("{}", &database_url);
    let mut opt = ConnectOptions::new(database_url.to_owned());
    opt.sqlx_logging(false);

    let db = Database::connect(opt).await?;

    Ok(Storage { db: Some(db) })
}

impl Storage {
    pub fn close(&self) {
        if let Some(conn) = &self.db {
            _ = conn.clone().close();
        }
    }
}

use std::env;

use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

use std::sync::{ Arc };

#[derive(Debug, Clone)]
pub struct AppState {
    db: Arc<MySqlPool>,
}

impl AppState {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        Self {
            db
        }
    }

    pub fn get_db(&self) -> &Arc<MySqlPool> {
        &self.db
    }
}

pub async fn setup_db() -> Result <MySqlPool, &'static str> {
    let mut db_url = String::from("mysql://");
    
    if let Ok(mysql_user) = env::var("MYSQL_USER") {
        db_url.push_str(&mysql_user);
    } else {
        return Err("Please set MYSQL_USER in your environment");
    }

    if let Ok(mysql_passwd) = env::var("MYSQL_PASSWD") {
        db_url.push_str(&format!(":{}", mysql_passwd));
    } else {
        return Err("Please set MYSQL_PASSWD in your environment");
    }

    if let Ok(mysql_addr) = env::var("MYSQL_ADDR") {
        db_url.push_str(&format!("@{}", mysql_addr));
    } else {
        return Err("Please set MYSQL_ADDR in your environment");
    }

    if let Ok(mysql_host) = env::var("MYSQL_HOST") {
        db_url.push_str(&format!(":{}", mysql_host));
    } else {
        return Err("Please set MYSQL_HOST in your environment");
    }

    if let Ok(mysql_db) = env::var("FM_STAGING_DB") {
        db_url.push_str(&format!("/{}", mysql_db));
    } else {
        return Err("Please set FM_STAGING_DB in your environment");
    }

    let pool_result = MySqlPoolOptions::new()
                        .max_connections(5)
                        .connect(db_url.as_str()).await;
    
    let pool;
    match pool_result {
        Ok(p) => {
            pool = p;
        },
        Err(e) => {
            eprintln!("Failed to create the pool {:#?}", e);
            return Err("invalid db_url failed to create the pool");
        }
    }
    
    println!("DB connection is successful");
    Ok(pool)
}

use mysql::*;
use mysql::prelude::*;
use std::env;

pub fn setup_db() -> Result <Pool, &'static str> {
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

    let pool_result = Pool::new(db_url.as_str());
    let mut pool: Pool;

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

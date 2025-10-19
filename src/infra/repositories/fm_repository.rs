use crate::domain::file::File;

use std::sync::Arc;
use sqlx::{ query, MySqlPool };

pub async fn get_files(
    limit: u8,
    offset: u64, 
    db_mut: &Arc<MySqlPool>
) -> Result<Vec<File>, &'static str> {
    let db = Arc::clone(db_mut);
    let files_result = sqlx::query_as!(
        File,
        r#"SELECT * FROM files ORDER BY id LIMIT ? OFFSET ?"#,
        limit,
        offset 
    ).fetch_all(&(*db)).await
    .map_err(|e| {
        eprintln!("(get_files QUERY_ERROR)::{:#?}", e);
        "Database error"
    });

    files_result

}

pub async fn upload_files(
    files: Vec<File>,
    db: &Arc<MySqlPool>
) -> Result<&'static str, &'static str> {
    if files.is_empty() {
        eprintln!("(ERROR):: No users to insert");
        return Err("EMPTY");
    }

    let mut values_clause = String::new();
    for i in 0..files.len() {
        values_clause.push_str("(?, ?, ?, ?)");
        if i != files.len() - 1 {
            values_clause.push_str(", ");
        }
    }

    let sql_query = format!("INSERT INTO files(id, name, location, f_type) VALUES {}", values_clause);

    let mut query_builder = query(&sql_query);

    for file in files {
        query_builder = query_builder
                            .bind(file.id)
                            .bind(file.name)
                            .bind(file.location)
                            .bind(file.f_type);
    }

    let db_clone = Arc::clone(db);
    match query_builder.execute(&(*db_clone)).await {
        Ok(result) => {
            return Ok("SUCCESS"); 
        },
        Err(e) => {
            eprintln!("failed to insert the file {:?}", e);
            return Err("ERROR");
        }
    }

}

pub async fn upload_file(
    file: File,
    db: &MySqlPool
) -> Result<&'static str, &'static str> {
    if file.is_empty() {
        return Err("EMPTY");
    }
    
    let result = sqlx::query!(
            "INSERT INTO files (id, name, location, f_type) 
            VALUES(?, ?, ?, ?)",
            file.id,
            file.name,
            file.location,
            file.f_type
        ).execute(db).await;

    match result {
        Ok(r) => {
            println!("Insert data succesful");
            return Ok("SUCCESS");
        },
        Err(e) => {
            eprintln!("(ERROR):: insert data failed");
            return Err("FAILED");
        }
    }
}

pub fn get_file() -> Result<File, String> {
    Err(String::from("something went wrong"))
}

pub fn create_files() -> Result<String, String> {
    Ok(String::from("File created"))
}

pub fn delete_files() -> Result<String, String> {
    Ok(String::from("files deleted"))
}

pub fn update_files() -> Result<String, String> {
    Ok(String::from("files updated"))
}

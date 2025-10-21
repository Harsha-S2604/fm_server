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


pub async fn get_file(
    file_id: u64, 
    db: &MySqlPool
) -> Result<File, &'static str> {
    let file_result = sqlx::query_as!(
        File,
        r#"SELECT * FROM files WHERE id=?"#,
        file_id
    ).fetch_optional(db).await
    .map_err(|e| {
        eprintln!("(get_file QUERY_ERROR)::{:#?}", e);
        "Database error"
    });

    match file_result {
        Ok(file_ok) => {
            match file_ok {
                Some(file) => Ok(file),
                None => Err("NOT_FOUND"),
            }
        }
        Err(e) => {
            eprintln!("(DB_ERROR):: {:?}", e);
            Err("DB_ERROR")
        },
    }
}

pub async fn upload_files(
    files: Vec<File>,
    db: &Arc<MySqlPool>
) -> Result<&'static str, &'static str> {
    if files.is_empty() {
        return Err("(DB_INSERT_ERROR):: files are empty to insert");
    }

    let mut values_clause = String::new();
    for i in 0..files.len() {
        values_clause.push_str("(?, ?, ?, ?, ?)");
        if i != files.len() - 1 {
            values_clause.push_str(", ");
        }
    }

    let sql_query = format!(
        "INSERT INTO files(id, name, location, f_type, user_id) VALUES {}", 
        values_clause
    );

    let mut query_builder = query(&sql_query);

    for file in files {
        query_builder = query_builder
                            .bind(file.id)
                            .bind(file.name)
                            .bind(file.location)
                            .bind(file.f_type)
                            .bind(file.user_id);
    }

    let db_clone = Arc::clone(db);
    match query_builder.execute(&(*db_clone)).await {
        Ok(_) => {
            return Ok("DB_INSERT_SUCCESS"); 
        },
        Err(e) => {
            eprintln!("(DB_INSERT_ERROR):: {:?}", e);
            return Err("DB_INSERT_ERROR");
        }
    }

}

pub async fn upload_file(
    file: File,
    db: &MySqlPool
) -> Result<&'static str, &'static str> {
    if file.is_empty() {
        return Err("EMPTY_DATA");
    }
    
    let result = sqlx::query!(
            "INSERT INTO files (id, name, location, f_type, user_id) 
            VALUES(?, ?, ?, ?, ?)",
            file.id,
            file.name,
            file.location,
            file.f_type,
            file.user_id
        ).execute(db).await;

    match result {
        Ok(_) => {
            return Ok("DB_INSERT_SUCCESS");
        },
        Err(e) => {
            eprintln!("(DB_INSERT_ERROR):: {:?}", e);
            return Err("DB_INSERT_ERROR");
        }
    }
}

pub async fn delete_file(
    file_id: u64,
    db: &MySqlPool
) -> Result<&'static str, &'static str> {
    let result = sqlx::query("DELETE FROM files WHERE id = ?")
                    .bind(file_id)
                    .execute(db)
                    .await;

    if let Err(res_err) = result {
        eprintln!("(DB_DELETE_ERROR):: {:?}", res_err);
        return Err("Failed to delete the data");
    }

    if let Ok(res) = result {
        if res.rows_affected() == 0 {
            return Err("invalid id - file not found");
        }
    }

    Ok("Deleted successfully")
}

use crate::domain::file::File;

use std::sync::Arc;
use sqlx::MySqlPool;

pub async fn get_files(
    limit: u8,
    offset: u64, 
    db_mut: &Arc<MySqlPool>
) -> Result<Vec<File>, &'static str> {
    let file = File {
        id: 123,
        name: String::from("file_1"),
        f_type: String::from("txt"),
        location: String::from("/home/user1/file.txt"),
    };
    
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

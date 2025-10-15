use crate::domain::file::{
    File,
};

pub fn get_files() -> Result<Vec<File>, String> {
    let file = File {
        id: 123,
        name: String::from("file_1"),
        f_type: String::from("txt"),
        location: String::from("/home/user1/file.txt"),
    };

    let files = vec![file];

    Ok(files)
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

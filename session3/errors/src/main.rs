use std::path::Path;
use serde::Deserialize;


fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("myfile.txt");
    let content = std::fs::read_to_string(my_file);
    return content;
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let content = maybe_read_a_file()?;
    return Ok(content.to_uppercase());
}

#[derive(Deserialize)]
struct User {
    user: String,
}

use thiserror::Error;

#[derive(Debug, Error)]
enum UsersError {
    #[error("No users found")]
    NoUsers,
    #[error("Too many users were found")]
    TooManyUsers,
}

fn load_users() -> Result<Vec<User>, UsersError> {
    let my_path = Path::new("users.json");

    let raw_text = std::fs::read_to_string(my_path).map_err(|_| { return UsersError::NoUsers; })?;

    let users: Vec<User> = serde_json::from_str(&raw_text).map_err(|_| { return UsersError::NoUsers; })?;

    return Ok(users);
}


fn main() {
    load_users();
    // match maybe_read_a_file() {
    //     Ok(_) => {}
    //     Err(error) => {
    //         match error.kind() {
    //             ErrorKind::NotFound => {
    //                 println!("Not found!")
    //             }
    //             _ => {}
    //         }
    //     }
    // }
}

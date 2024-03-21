use std::collections::HashMap;
use std::path::Path;
use project_root::get_project_root;

use serde::{Deserialize, Serialize};

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);

    return format!("{:X}", hasher.finalize());
}

pub fn greet_user(name: &str) -> String {
    return format!("Hello {name}");
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
    Denied,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> Self {
        return Self {
            username: username.to_string(),
            password: hash_password(password),
            role,
        };
    }
}


pub fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();

    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("bob".to_string(), User::new("bob", "password", LoginRole::User));

    return users;
}

pub fn save_users(users: HashMap<String, User>) {
    let project_root = get_project_root().unwrap();
    let users_path = project_root.join(Path::new("users.json"));

    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(users_path, users_json).unwrap();
}

pub fn get_users() -> HashMap<String, User> {
    let project_root = get_project_root().unwrap();
    let users_path = project_root.join(Path::new("users.json"));


    return if users_path.exists() {
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json.as_str()).unwrap();
        return users;
    } else {
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    };
}

fn get_admin_users() {
    let users: Vec<User> = get_users().into_iter().filter(|(username, user)| {
        return user.role == LoginRole::Admin;
    }).map(|(username, user)| user).collect();
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();

    let hashed_password = hash_password(password);

    if let Some(found_user) = users.get(username) {
        dbg!(password, &hashed_password, &found_user, &users);


        if found_user.password == hashed_password {
            return Some(LoginAction::Granted(found_user.role.clone()));
        }

        println!("Denied!");
        return Some(LoginAction::Denied);
    };

    return None;
}

pub fn read_line() -> String {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");

    return input.trim().to_string();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Wojciech", greet_user("Wojciech"))
    }

    #[test]
    fn test_login() {
        assert_eq!(login("admin", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!(login("Admin", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!(login("bob", "password"), Some(LoginAction::Granted(LoginRole::User)));

        assert_eq!(login("admin", "passwrd"), Some(LoginAction::Denied));
        assert_eq!(login("admin1", "password"), None);
    }
}

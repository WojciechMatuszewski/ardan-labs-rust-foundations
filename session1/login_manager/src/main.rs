use clap::{Parser, Subcommand};
use authentication::{get_users, LoginRole, save_users, User};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
    /// Add a user.
    Add {
        username: String,
        password: String,
        #[arg(long)]
        admin: Option<bool>,
    },
    Delete {
        username: String
    },
    ChangePassword {
        username: String,

        new_password: String,
    },
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Password");
    println!("{:-<40}", "");

    get_users()
        .iter()
        .for_each(|(_, user)| {
            println!("{:<20}{:20?}", user.username, user.role)
        });
}

fn add_user(username: String, password: String, admin: bool) {
    let mut users = get_users();
    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };

    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(users);
}

fn delete_user(username: String) {
    let mut users = get_users();
    if let None = users.remove(&username) {
        println!("{username} does not exist");
    } else {
        save_users(users);
    }
}

fn change_password(username: String, new_password: String) {
    let mut users = get_users();
    if let Some(user) = users.get_mut(&username) {
        user.password = authentication::hash_password(&new_password);
        save_users(users);
    } else {
        println!("{username} does not exist");
    }
}


fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        Some(Commands::Add { username, password, admin }) => {
            add_user(username, password, admin.unwrap_or(false));
        }
        Some(Commands::Delete { username }) => {
            delete_user(username);
        }
        Some(Commands::ChangePassword { username, new_password }) => {
            change_password(username, new_password.trim().to_string())
        }
        None => {}
    }
}

use authentication::{login, LoginAction, read_line};

fn main() {
    let mut tries = 0;

    loop {
        println!("Enter your username:");
        let username = read_line();

        println!("Enter your password:");
        let password = read_line();

        match login(&username, &password) {
            None => {
                println!("Incorrect name");
            }
            Some(login_action) => {
                match login_action {
                    LoginAction::Granted(role) => {
                        println!("Welcome {role:?}");
                        break;
                    }
                    LoginAction::Denied => {
                        println!("Incorrect password");
                    }
                }
            }
        }

        tries += 1;
        if tries >= 3 {
            println!("Too many failed logins!");
            break;
        }
    }
}

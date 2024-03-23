use std::sync::RwLock;
use once_cell::sync::Lazy;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| {
    return RwLock::new(build_users());
});

fn build_users() -> Vec<String> {
    return vec![
        "Alice".to_string(),
        "Bob".to_string(),
    ];
}

fn read_line() -> String {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    return input.trim().to_string();
}


fn main() {
    std::thread::spawn(|| {
        loop {
            println!("Current users (in a thread)");
            let users = USERS.read().unwrap();
            println!("{users:?}");
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });

    loop {
        println!("Enter a name to add the user list (or q to quit)");


        let input = read_line();

        if input == "q" {
            break;
        }

        let mut lock = USERS.write().unwrap();
        lock.push(input);
    }
}

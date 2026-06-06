use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

const DB_PATH: &str = "client/db";
const DB: &str = "client/db/users.json";

// Load users. If the file is missing or empty, just start fresh.
fn load_users() -> Vec<User> {
    match fs::read_to_string(DB) {
        Ok(contents) if !contents.trim().is_empty() => {
            serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
        }
        _ => Vec::new(),
    }
}

// Save the whole list back, creating the db/ folder if needed.
fn save_users(users: &[User]) {
    fs::create_dir_all(DB_PATH).expect("Unable to create db folder");
    let json = serde_json::to_string_pretty(users).expect("Unable to serialize");
    fs::write(DB, json).expect("Unable to write file");
}

fn register_user() {
    println!("Registering user...");

    let mut username = String::new();
    println!("Enter username: ");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    let mut password = String::new();
    println!("Enter password: ");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    let username = username.trim().to_string();
    let mut users = load_users();

    // The "username taken" check we talked about
    if users.iter().any(|u| u.username == username) {
        println!("That username is already taken!");
        return;
    }

    users.push(User {
        username,
        password: password.trim().to_string(),
    });

    save_users(&users);
    println!("Registered!");
}

fn login_user() {
    println!("Logging in...");

    let mut username = String::new();
    println!("Enter username: ");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    let mut password = String::new();
    println!("Enter password: ");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    let username = username.trim();
    let password = password.trim();

    let users = load_users();

    if users.iter().any(|u| u.username == username && u.password == password) {
        println!("Login successful!");
    } else {
        println!("Invalid username or password.");
    }
}   

fn main_menu() {
    println!("Welcome to ProjectR");
    println!("1. Login");
    println!("2. Register");
    println!("3. Exit");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => login_user(  ),
        "2" => register_user(),
        "3" => println!("Exit selected"),
        _ => println!("Invalid choice"),
    }
}

fn main() {
    main_menu();
}
// use bevy::prelude::*;

// fn main() {

//     App::new()
//         .add_plugins(DefaultPlugins)
//         .run();
// }

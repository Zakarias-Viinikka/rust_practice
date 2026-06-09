use std::io;
use std::process::Command;

fn main() {
    clear_console();
    loop {
        display_main_menu();
        match read_line() {
            Ok(x) if x == "quit".to_string() => {
                break;
            }
            Ok(x) if x == "Q".to_string() => {
                go_to_special_menu();
            }
            Ok(x) => show_what_user_wrote(x),
            Err(e) => println!("{e}"),
        }
    }
}

fn go_to_special_menu() {
    clear_console();
    loop {
        display_special_menu();
        match read_line() {
            Ok(x) if x == "b".to_string() => {
                clear_console();
                break;
            }
            Ok(x) => show_what_user_wrote(x),
            Err(e) => println!("{e}"),
        }
    }
}

fn display_main_menu() {
    println!("Main Menu");
    println!("Commands:");
    println!("'Q' to enter menu 1");
    println!("'quit' to exit program");

    println!("");
    println!("waiting for user input:");
}

fn display_special_menu() {
    println!("Special Menu");
    println!("Commands:");
    println!("'b' to go back");
    println!(
        "just like the main menu you can also type anything and it will show you what you wrote."
    );

    println!("");
    println!("waiting for user input:");
}

fn show_what_user_wrote(user_input: String) {
    clear_console();
    println!("user wrote this: {user_input}");
    println!("")
}

fn read_line() -> io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().ok();
    } else {
        Command::new("clear").status().ok();
    }
}

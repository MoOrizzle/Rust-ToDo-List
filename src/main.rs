mod json;
mod list_entry;

use std::{env, fs, io::{self, Write}, path::PathBuf, process::Command};
use chrono::Local;

use crate::list_entry::ListEntry;

const ENTRIES_PATH: &str = "./data/entries.json";

fn main() {
    let path: PathBuf = PathBuf::from(ENTRIES_PATH);
    if !path.exists() {
        fs::File::create(&path).expect("Couldn't create data file");
    }

    println!("Hallo! Was möchtest du tun?");
    loop {
        println!();
        println!("1: Einträge ansehen.");
        println!("2: Einträge hinzufügen.");
        println!("3: Einträge löschen.");
        println!();

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input = user_input.trim();
        if matches!(user_input, "q" | "quit" | "exit") {
            std::process::exit(1)
        }

        let user_input: u8 = match user_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input, try again");
                continue;
            }  
        };
        
        clear_console();
        match user_input {
            1 => {
                print_todo_list(&path);
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                clear_console();
            },
            2 => {
                println!("Neuer Eintrag wird erstellt...");
                add_todo_entry(&path);
                clear_console();
                println!("Neuer Eintrag wurde erfolgreich erstellt.")
            },
            3 => {
                print_todo_list(&path);
                println!("Welchen Eintrag möchtest du löschen?");
                remove_entry_from_list(&path);
                clear_console();
                println!("Eintrage wurde erfolgreich gelöscht!");
                print_todo_list(&path);
            },
            _ => println!("Invalid Operation.")
        }
    }
}

fn print_todo_list(path: &PathBuf) {
    let entries = json::load_from_json(path);

    if entries.len() == 0 {
        println!("Keine Einträge vorhanden.");
        return;
    }

    for (index, entry) in entries.iter().enumerate()  {
        let formatted = entry.time_stamp.format("%d.%m.%Y um %H:%M Uhr");
        println!("{}. \"{}\" - {}:", index + 1, entry.title, formatted);
        println!("   {}", entry.text)
    }
    println!()
}

fn add_todo_entry(path: &PathBuf) {

    let mut title_buffer = String::new();
    print!("Title: ");

    io::stdout()
        .flush()
        .expect("Failed to flush text.");

    io::stdin()
        .read_line(&mut title_buffer)
        .expect("Failed to read line");

    let mut text_buffer = String::new();
    print!("Text: ");

    io::stdout()
        .flush()
        .expect("Failed to flush text.");

    io::stdin()
        .read_line(&mut text_buffer)
        .expect("Failed to read line");

    let entry = ListEntry {
        time_stamp: Local::now(),
        title: title_buffer.trim().to_string(),
        text: text_buffer.trim().to_string()
    };
    json::append_entry_to_json(path, entry);
}

fn remove_entry_from_list(path: &PathBuf) {
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: u8 = index
        .trim()
        .parse()
        .expect("Couldn't parse user input to valid number");

    let index: usize = (index - 1).into();

    let mut entries = json::load_from_json(path);
    entries.remove(index);

    json::remove_entry_from_json(path, index);
}

fn clear_console() {
    let mut command: &str = "clear";
    if env::consts::OS == "windows" {
        command = "cls";
    }
    Command::new(command).status().expect("Couldn't clear console");
}

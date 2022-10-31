extern crate chrono;
extern crate dirs;
use chrono::{Local, DateTime};
use dirs::home_dir;
use std::fs::{create_dir, OpenOptions};
use std::path::PathBuf;
use std::io::Write;

fn main() {
    let dir = home_dir().expect("Could not find home directory");
    let notes_path_temp = dir.join(".notes");
    let notes_path = notes_path_temp.clone();

    if !notes_path_temp.exists() {
        create_dir(notes_path_temp).expect("Unable to create notes directory");
    }

    let now: DateTime<Local> = Local::now();
    let notes_file_name = format!("notes-{}.txt", now.format("%Y-%m-%d"));
    let notes_file_path = notes_path.join(notes_file_name);

    if !notes_file_path.exists() {
        let _ = OpenOptions::new()
                  .read(true)
                  .write(true)
                  .create(true)
                  .open(notes_file_path.to_str()
                  .expect("Could not create notes file"));
    }

    let mut args = std::env::args();
    if args.len() == 2 {
        create_new_note(&notes_file_path, &args.nth(1).unwrap());
    } else {
        display_notes(&notes_file_path);
    }
}

fn display_notes(notes_path: &PathBuf) {
    let file_contents = std::fs::read_to_string(notes_path).expect("Error reading notes file.");
    println!("{}", file_contents);
}

fn create_new_note(notes_file_path: &PathBuf, new_note: &String) {
    let mut file = OpenOptions::new().write(true).append(true).open(notes_file_path).unwrap();
    let now: DateTime<Local> = Local::now();
    let new_line = format!("{} - {}\n", now.format("%Y-%m-%d %H:%M:%S"), new_note);
    file.write(new_line.as_bytes()).unwrap();
}

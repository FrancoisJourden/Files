// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use tauri::api::path::home_dir;

#[derive(serde::Serialize)]
struct File {
    name: String,
    is_dir: bool,
    path: String,
}

// a function that takes a directory path as input and return a list of files in the directory as str list
#[tauri::command]
fn read_dir(path: &str, with_hidden: Option<bool>) -> Option<Vec<File>> {
    let mut result: Vec<File> = Vec::new();

    let dir = fs::read_dir(path);


    if dir.is_err() {
        return None;
    }

    for item in dir.unwrap() {
        let item_unwrapped = item.unwrap();
        let file_name = item_unwrapped.file_name().into_string().unwrap();

        if (with_hidden.is_none() || !with_hidden.unwrap()) && file_name.starts_with(".") {
            continue;
        }

        result.push(File {
            name: file_name.clone(),
            is_dir: item_unwrapped.metadata().unwrap().is_dir(),
            path: item_unwrapped.path().as_path().to_str().unwrap().to_string(),
        });
    }

    return Some(result);
}

#[tauri::command]
fn home_dir_path() -> String{
    return home_dir().unwrap().to_str().unwrap().to_string();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_dir, home_dir_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

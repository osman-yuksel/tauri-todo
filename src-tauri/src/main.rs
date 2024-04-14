// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(serde::Serialize, Debug)]
struct Todo {
    id: u32,
    value: String,
}

impl Clone for Todo {
    fn clone(&self) -> Self {
        Todo {
            id: self.id,
            value: self.value.clone(),
        }
    }
}

#[derive(Default)]
struct Database(Arc<Mutex<HashMap<u32, Todo>>>);

#[tauri::command]
fn todos_get(db: tauri::State<'_, Database>) -> Result<Vec<Todo>, String> {
    let db = db.0.lock().unwrap();
    Ok(db.values().cloned().collect())
}

#[tauri::command]
fn todo_add(value: &str, db: tauri::State<'_, Database>) -> Result<Todo, String> {
    let mut db = db.0.lock().unwrap();
    let id = db.len() as u32 + 1;
    let todo = Todo {
        id,
        value: value.to_string(),
    };
    db.insert(id, todo.clone());
    println!("TODO:ADD {:?}", todo);
    Ok(todo.clone())
}

fn main() {
    tauri::Builder::default()
        .manage(Database(Default::default()))
        .invoke_handler(tauri::generate_handler![todos_get, todo_add])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

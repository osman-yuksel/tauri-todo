// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::Connection;
use std::sync::{Arc, Mutex};

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
struct Database(Arc<Mutex<Connection>>);

impl Database {
    fn new() -> Self {
        Database(Arc::new(Mutex::new(Database::create_connection())))
    }

    fn create_connection() -> Connection {
        let conn = Connection::open("todo.db3").expect("Error while opening database");
        let table = conn.execute(
            "CREATE TABLE todo (
            id    INTEGER PRIMARY KEY,
            value TEXT 
        )",
            (),
        );

        match table {
            Ok(_) => println!("Table created!"),
            Err(rusqlite::Error::SqlInputError {
                error,
                msg,
                sql: _,
                offset: _,
            }) => println!("Error while creating table {:?}: {:?}", error, msg),
            Err(err) => println!("SQLITE ERROR {:?}", err),
        }

        conn
    }
}

#[tauri::command]
fn todos_get(db: tauri::State<'_, Database>) -> Result<Vec<Todo>, String> {
    let db = db.0.lock().unwrap();
    let mut stmt = db.prepare("SELECT id, value FROM todo").unwrap();
    let todos = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                value: row.get(1)?,
            })
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    Ok(todos)
}

#[tauri::command]
fn todo_add(value: &str, db: tauri::State<'_, Database>) -> Result<Todo, String> {
    let db = db.0.lock().unwrap();
    let entry = db.execute("INSERT INTO todo (value) VALUES (?)", &[&value.to_string()]);

    match entry {
        Ok(_) => println!("TODO:ADD {:?}", entry),
        Err(err) => println!("Error while adding todo {:?}", err),
    };

    Ok(Todo {
        id: db.last_insert_rowid() as u32,
        value: value.to_string(),
    })
}

#[tauri::command]
fn todo_delete(value: u32, db: tauri::State<'_, Database>) -> Result<(), String> {
    let db = db.0.lock().unwrap();
    let entry = db.execute("DELETE FROM todo WHERE id = ?", &[&value]);

    match entry {
        Ok(_) => println!("TODO:REMOVE {:?}", entry),
        Err(err) => println!("Error while removing todo {:?}", err),
    };

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(Database::new())
        .invoke_handler(tauri::generate_handler![todos_get, todo_add, todo_delete])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

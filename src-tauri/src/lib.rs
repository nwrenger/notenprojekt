pub mod db;
pub mod error;

use std::fs;

use error::{Error, Result};
use tauri::{async_runtime::RwLock, Manager, State};

use crate::db::{Database, Fach, Note, Zeitraum};

#[tauri::command]
async fn get_zeitraeume(db: State<'_, RwLock<Database>>) -> Result<Vec<Zeitraum>> {
    db.write().await.get_zeitraeume().await
}

#[tauri::command]
async fn get_faecher(db: State<'_, RwLock<Database>>) -> Result<Vec<Fach>> {
    db.write().await.get_faecher().await
}

#[tauri::command]
async fn get_noten(zeitraum_id: i64, db: State<'_, RwLock<Database>>) -> Result<Vec<Note>> {
    db.write().await.get_noten(zeitraum_id).await
}

#[tauri::command]
async fn add_zeitraum(quartal: i64, stufe: i64, db: State<'_, RwLock<Database>>) -> Result<()> {
    db.write().await.add_zeitraum(quartal, stufe).await
}

#[tauri::command]
async fn add_fach(
    name: String,
    lehrer: Option<String>,
    db: State<'_, RwLock<Database>>,
) -> Result<()> {
    db.write().await.add_fach(name, lehrer).await
}

#[tauri::command]
async fn add_note(
    zeitraum_id: i64,
    fach_id: i64,
    schriftlich: Option<i64>,
    muendlich: Option<i64>,
    gewichtung: f64,
    db: State<'_, RwLock<Database>>,
) -> Result<()> {
    db.write()
        .await
        .add_note(zeitraum_id, fach_id, schriftlich, muendlich, gewichtung)
        .await
}

#[tauri::command]
async fn edit_zeitraum(
    id: i64,
    quartal: i64,
    stufe: i64,
    db: State<'_, RwLock<Database>>,
) -> Result<()> {
    db.write().await.edit_zeitraum(id, quartal, stufe).await
}

#[tauri::command]
async fn edit_fach(
    id: i64,
    name: String,
    lehrer: Option<String>,
    db: State<'_, RwLock<Database>>,
) -> Result<()> {
    db.write().await.edit_fach(id, name, lehrer).await
}

#[tauri::command]
async fn edit_note(
    id: i64,
    fach_id: i64,
    schriftlich: Option<i64>,
    muendlich: Option<i64>,
    gewichtung: f64,
    db: State<'_, RwLock<Database>>,
) -> Result<()> {
    db.write()
        .await
        .edit_note(id, fach_id, schriftlich, muendlich, gewichtung)
        .await
}

#[tauri::command]
async fn delete_zeitraum(zeitraum_id: i64, db: State<'_, RwLock<Database>>) -> Result<()> {
    db.write().await.delete_zeitraum(zeitraum_id).await
}

#[tauri::command]
async fn delete_fach(fach_id: i64, db: State<'_, RwLock<Database>>) -> Result<()> {
    db.write().await.delete_fach(fach_id).await
}

#[tauri::command]
async fn delete_note(note_id: i64, db: State<'_, RwLock<Database>>) -> Result<()> {
    db.write().await.delete_note(note_id).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Using the add data dir
            let app_data_dir = app.path().app_data_dir()?;

            // Make sure that the app dir exists
            if !app_data_dir.exists() {
                fs::create_dir_all(&app_data_dir).map_err(|_| tauri::Error::UnknownPath)?;
            }

            tauri::async_runtime::block_on(async move {
                // Connect to db, creates one if there doesn't any exist
                let db = Database::connect(&app_data_dir.join("noten.db")).await?;
                app.manage(RwLock::new(db));

                Ok::<(), Error>(())
            })?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_zeitraeume,
            get_faecher,
            get_noten,
            add_zeitraum,
            add_fach,
            add_note,
            edit_zeitraum,
            edit_fach,
            edit_note,
            delete_zeitraum,
            delete_fach,
            delete_note
        ])
        .build(tauri::generate_context!())
        .expect("error while building clave application")
        .run(|app_handle, event| {
            // Clean Up the Database Connection
            if let tauri::RunEvent::Exit = event {
                #[allow(deprecated)]
                app_handle.unmanage::<RwLock<Database>>();
            }
        });
}

pub mod error;

use std::{fs, sync::Mutex};

use error::Result;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

#[derive(Debug, Serialize, Deserialize)]
struct Fach {
    id: usize,
    name: String,
    lehrer: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    id: usize,
    muendlich: Option<usize>,
    schriftlich: Option<usize>,
    gewichtung: f32,
    insgesamt: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Zeitraum {
    id: usize,
    quartal: usize,
    stufe: usize,
}

#[tauri::command]
fn get_zeitraeume(conn: State<Mutex<Connection>>) -> Result<Vec<Zeitraum>> {
    let guard = conn.lock()?;
    let mut stmt = guard.prepare("SELECT id, quartal, stufe FROM Zeitraum")?;

    let iter = stmt.query_map([], |row| {
        Ok(Zeitraum {
            id: row.get(0)?,
            quartal: row.get(1)?,
            stufe: row.get(2)?,
        })
    })?;

    let mut result = vec![];

    for zeitraum in iter {
        result.push(zeitraum?);
    }

    Ok(result)
}

#[tauri::command]
fn get_faecher(conn: State<Mutex<Connection>>) -> Result<Vec<Fach>> {
    let guard = conn.lock()?;
    let mut stmt = guard.prepare("SELECT id, name, lehrer FROM Fach")?;

    let iter = stmt.query_map([], |row| {
        Ok(Fach {
            id: row.get(0)?,
            name: row.get(1)?,
            lehrer: row.get(2)?,
        })
    })?;

    let mut result = vec![];

    for fach in iter {
        result.push(fach?);
    }

    Ok(result)
}

#[tauri::command]
fn get_fachnoten_by_zeitraum(
    zeitraum_id: usize,
    conn: State<Mutex<Connection>>,
) -> Result<Vec<(Fach, Note)>> {
    let guard = conn.lock()?;
    let mut stmt = guard.prepare(
        "SELECT Fach.id, Fach.name, Fach.lehrer, Note.id, Note.schriftlich, Note.muendlich, Note.gewichtung, Note.insgesamt
   FROM ZeitraumHatNote
   JOIN Note ON Note.id = ZeitraumHatNote.note_id
   JOIN Fach ON Fach.id = ZeitraumHatNote.fach_id
   WHERE ZeitraumHatNote.zeitraum_id = ?1",
    )?;

    let iter = stmt.query_map(params![zeitraum_id], |row| {
        Ok((
            Fach {
                id: row.get(0)?,
                name: row.get(1)?,
                lehrer: row.get(2)?,
            },
            Note {
                id: row.get(3)?,
                schriftlich: row.get(4)?,
                muendlich: row.get(5)?,
                gewichtung: row.get(6)?,
                insgesamt: row.get(7)?,
            },
        ))
    })?;

    let mut result = vec![];

    for fach in iter {
        result.push(fach?);
    }

    Ok(result)
}

#[tauri::command]
fn add_zeitraum(quartal: usize, stufe: usize, conn: State<Mutex<Connection>>) -> Result<()> {
    let guard = conn.lock()?;
    guard.execute(
        "INSERT INTO Zeitraum (quartal, stufe) VALUES (?1, ?2)",
        params![quartal, stufe],
    )?;

    Ok(())
}

#[tauri::command]
fn add_fach(name: String, lehrer: String, conn: State<Mutex<Connection>>) -> Result<()> {
    let guard = conn.lock()?;
    guard.execute(
        "INSERT INTO Fach (name, lehrer) VALUES (?1, ?2)",
        params![name, lehrer],
    )?;

    Ok(())
}

#[tauri::command]
fn add_note(
    zeitraum_id: usize,
    fach_id: usize,
    schriftlich: Option<usize>,
    muendlich: Option<usize>,
    gewichtung: f32,
    conn: State<Mutex<Connection>>,
) -> Result<()> {
    let insgesamt = match (schriftlich, muendlich) {
        (Some(s), Some(m)) => Some((s as f32) * (1.0 - gewichtung) + (m as f32) * gewichtung),
        _ => None,
    };

    let mut guard = conn.lock()?;
    let tx = guard.transaction()?;

    tx.execute(
        "INSERT INTO Note (schriftlich, muendlich, gewichtung, insgesamt)
         VALUES (?1, ?2, ?3, ?4)",
        params![schriftlich, muendlich, gewichtung, insgesamt],
    )?;

    let note_id = tx.last_insert_rowid() as usize;
    tx.execute(
        "INSERT INTO ZeitraumHatNote (zeitraum_id, fach_id, note_id)
         VALUES (?1,          ?2,      ?3      )",
        params![zeitraum_id, fach_id, note_id],
    )?;

    tx.commit()?;
    Ok(())
}

#[tauri::command]
fn edit_zeitraum(
    id: usize,
    quartal: usize,
    stufe: usize,
    conn_state: State<Mutex<Connection>>,
) -> Result<()> {
    let conn = conn_state.lock()?;
    conn.execute(
        "UPDATE Zeitraum SET quartal = ?1, stufe = ?2 WHERE id = ?3",
        params![quartal, stufe, id],
    )?;
    Ok(())
}

#[tauri::command]
fn edit_fach(
    id: usize,
    name: String,
    lehrer: String,
    conn_state: State<Mutex<Connection>>,
) -> Result<()> {
    let conn = conn_state.lock()?;
    conn.execute(
        "UPDATE Fach SET name = ?1, lehrer = ?2 WHERE id = ?3",
        params![name, lehrer, id],
    )?;
    Ok(())
}

#[tauri::command]
fn edit_note(
    id: usize,
    fach_id: usize,
    schriftlich: Option<usize>,
    muendlich: Option<usize>,
    gewichtung: f32,
    conn: State<Mutex<Connection>>,
) -> Result<()> {
    let insgesamt = match (schriftlich, muendlich) {
        (Some(s), Some(m)) => Some((s as f32) * (1.0 - gewichtung) + (m as f32) * gewichtung),
        _ => None,
    };
    let mut guard = conn.lock()?;
    let tx = guard.transaction()?;

    tx.execute(
        "UPDATE Note SET schriftlich = ?1, muendlich = ?2, gewichtung = ?3, insgesamt = ?4 WHERE id = ?5",
        params![schriftlich, muendlich, gewichtung, insgesamt, id],
    )?;

    tx.execute(
        "UPDATE ZeitraumHatNote SET fach_id = ?1 WHERE note_id = ?2",
        params![fach_id, id],
    )?;

    Ok(tx.commit()?)
}

#[tauri::command]
fn delete_zeitraum(zeitraum_id: usize, conn: State<Mutex<Connection>>) -> Result<()> {
    let mut guard = conn.lock().unwrap();
    let tx = guard.transaction()?;

    // Get all relations
    let note_ids: Vec<i64> = {
        let mut stmt = tx.prepare("SELECT note_id FROM ZeitraumHatNote WHERE zeitraum_id = ?1")?;
        let iter = stmt.query_map(params![zeitraum_id], |row| row.get::<_, i64>(0))?;

        let mut ids = Vec::new();
        for note in iter {
            ids.push(note?);
        }
        ids
    };

    // Delete from the relations-table
    tx.execute(
        "DELETE FROM ZeitraumHatNote WHERE zeitraum_id = ?1",
        params![zeitraum_id],
    )?;

    // Delete all notes with relations
    if !note_ids.is_empty() {
        let mut del = tx.prepare("DELETE FROM Note WHERE id = ?1")?;
        for nid in &note_ids {
            del.execute(params![nid])?;
        }
    }

    // Delete Zeitraum
    tx.execute("DELETE FROM Zeitraum WHERE id = ?1", params![zeitraum_id])?;

    Ok(tx.commit()?)
}

#[tauri::command]
fn delete_fach(fach_id: usize, conn: State<Mutex<Connection>>) -> Result<()> {
    let mut guard = conn.lock().unwrap();
    let tx = guard.transaction()?;

    // Get all relations
    let note_ids: Vec<i64> = {
        let mut stmt = tx.prepare("SELECT note_id FROM ZeitraumHatNote WHERE fach_id = ?1")?;
        let iter = stmt.query_map(params![fach_id], |row| row.get::<_, i64>(0))?;

        let mut ids = Vec::new();
        for note in iter {
            ids.push(note?);
        }
        ids
    };

    // Delete from the relations-table
    tx.execute(
        "DELETE FROM ZeitraumHatNote WHERE fach_id = ?1",
        params![fach_id],
    )?;

    // Delete all notes with relations
    if !note_ids.is_empty() {
        let mut del = tx.prepare("DELETE FROM Note WHERE id = ?1")?;
        for nid in &note_ids {
            del.execute(params![nid])?;
        }
    }

    // Delete Fach
    tx.execute("DELETE FROM Fach WHERE id = ?1", params![fach_id])?;

    Ok(tx.commit()?)
}

#[tauri::command]
fn delete_note(note_id: usize, conn: State<Mutex<Connection>>) -> Result<()> {
    let mut guard = conn.lock()?;
    let tx = guard.transaction()?;

    tx.execute(
        "DELETE FROM ZeitraumHatNote WHERE note_id = ?1",
        params![note_id],
    )?;
    tx.execute("DELETE FROM Note WHERE id = ?1", params![note_id])?;

    Ok(tx.commit()?)
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

            // Connect to db, creates one if there doesn't any exist
            let conn = Connection::open(app_data_dir.join("noten.db"))?;

            // setup tables
            conn.execute_batch(
                r#"
                CREATE TABLE IF NOT EXISTS "Fach" (
                   	"id"	INTEGER NOT NULL,
                   	"name"	TEXT NOT NULL,
                   	"lehrer"	TEXT NOT NULL,
                   	PRIMARY KEY("id" AUTOINCREMENT)
                );
                CREATE TABLE IF NOT EXISTS "Note" (
                   	"id"	INTEGER NOT NULL,
                   	"muendlich"	INTEGER,
                   	"schriftlich"	INTEGER,
                   	"gewichtung"	INTEGER NOT NULL,
                   	"insgesamt"	INTEGER,
               	PRIMARY KEY("id" AUTOINCREMENT)
                );
                    CREATE TABLE IF NOT EXISTS "Zeitraum" (
                   	"id"	INTEGER NOT NULL,
                   	"quartal" INTEGER NOT NULL,
                   	"stufe" INTEGER NOT NULL,
                   	PRIMARY KEY("id" AUTOINCREMENT)
                );
                    CREATE TABLE IF NOT EXISTS "ZeitraumHatNote" (
                   	"zeitraum_id"	INTEGER NOT NULL,
                   	"fach_id"	INTEGER NOT NULL,
                   	"note_id"	INTEGER NOT NULL,
                   	PRIMARY KEY("zeitraum_id","fach_id","note_id")
                );"#,
            )?;

            app.manage(Mutex::new(conn));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_zeitraeume,
            get_faecher,
            get_fachnoten_by_zeitraum,
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

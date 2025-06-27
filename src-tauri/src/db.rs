use std::path::Path;

use crate::error::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Connection, SqliteConnection};

/// The Zeitraum table
#[derive(Debug, Serialize, Deserialize)]
pub struct Zeitraum {
    id: i64,
    quartal: i64,
    stufe: i64,
}

/// The Fach table
#[derive(Debug, Serialize, Deserialize)]
pub struct Fach {
    id: i64,
    name: String,
    lehrer: Option<String>,
}

/// The Note table (with one exception, the `fach_id` field)
#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    id: i64,
    /// This field does not exist on the note table.
    /// It's from joining via the relations-table and needed for referencing related subject information.
    fach_id: i64,
    muendlich: Option<i64>,
    schriftlich: Option<i64>,
    gewichtung: f64,
    insgesamt: Option<f64>,
}

// Helper struct
struct NoteId {
    note_id: i64,
}

/// A wrapper with the database functions over the connection.
pub struct Database {
    db: SqliteConnection,
}

impl Database {
    pub fn new(db: SqliteConnection) -> Self {
        Self { db }
    }

    pub async fn connect(path: &Path) -> Result<Self> {
        let path_str = path.to_string_lossy();
        let mut db = SqliteConnection::connect(&format!("sqlite://{path_str}")).await?;

        query!(
            r#"
        CREATE TABLE IF NOT EXISTS "Fach" (
           	"id"	INTEGER NOT NULL,
           	"name"	TEXT NOT NULL,
           	"lehrer"	TEXT,
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
        )
        .execute(&mut db)
        .await?;

        Ok(Database::new(db))
    }

    pub async fn get_zeitraeume(&mut self) -> Result<Vec<Zeitraum>> {
        Ok(
            query_as!(Zeitraum, "SELECT id, quartal, stufe FROM Zeitraum")
                .fetch_all(&mut self.db)
                .await?,
        )
    }

    pub async fn get_faecher(&mut self) -> Result<Vec<Fach>> {
        Ok(query_as!(Fach, "SELECT id, name, lehrer FROM Fach")
            .fetch_all(&mut self.db)
            .await?)
    }

    pub async fn get_noten(&mut self, zeitraum_id: i64) -> Result<Vec<Note>> {
        Ok(query_as!(Note, "SELECT Note.id, Fach.id as fach_id, Note.muendlich, Note.schriftlich, Note.gewichtung, Note.insgesamt
   FROM ZeitraumHatNote
   JOIN Note ON Note.id = ZeitraumHatNote.note_id
   JOIN Fach ON Fach.id = ZeitraumHatNote.fach_id
   WHERE ZeitraumHatNote.zeitraum_id = ?1", zeitraum_id)
            .fetch_all(&mut self.db)
            .await?)
    }

    pub async fn add_zeitraum(&mut self, quartal: i64, stufe: i64) -> Result<()> {
        query!(
            "INSERT INTO Zeitraum (quartal, stufe) VALUES (?1, ?2)",
            quartal,
            stufe,
        )
        .execute(&mut self.db)
        .await?;

        Ok(())
    }

    pub async fn add_fach(&mut self, name: String, lehrer: Option<String>) -> Result<()> {
        query!(
            "INSERT INTO Fach (name, lehrer) VALUES (?1, ?2)",
            name,
            lehrer,
        )
        .execute(&mut self.db)
        .await?;

        Ok(())
    }

    pub async fn add_note(
        &mut self,
        zeitraum_id: i64,
        fach_id: i64,
        schriftlich: Option<i64>,
        muendlich: Option<i64>,
        gewichtung: f64,
    ) -> Result<()> {
        let insgesamt = match (schriftlich, muendlich) {
            (Some(s), Some(m)) => Some((s as f64) * (1.0 - gewichtung) + (m as f64) * gewichtung),
            _ => None,
        };

        let note_id = query!(
            "INSERT INTO Note (schriftlich, muendlich, gewichtung, insgesamt)
             VALUES (?1, ?2, ?3, ?4)",
            schriftlich,
            muendlich,
            gewichtung,
            insgesamt
        )
        .execute(&mut self.db)
        .await?
        .last_insert_rowid();

        query!(
            "INSERT INTO ZeitraumHatNote (zeitraum_id, fach_id, note_id)
             VALUES (?1, ?2, ?3)",
            zeitraum_id,
            fach_id,
            note_id,
        )
        .execute(&mut self.db)
        .await?;

        Ok(())
    }

    pub async fn edit_zeitraum(&mut self, id: i64, quartal: i64, stufe: i64) -> Result<()> {
        query!(
            "UPDATE Zeitraum SET quartal = ?1, stufe = ?2 WHERE id = ?3",
            quartal,
            stufe,
            id,
        )
        .execute(&mut self.db)
        .await?;

        Ok(())
    }

    pub async fn edit_fach(&mut self, id: i64, name: String, lehrer: Option<String>) -> Result<()> {
        query!(
            "UPDATE Fach SET name = ?1, lehrer = ?2 WHERE id = ?3",
            name,
            lehrer,
            id,
        )
        .execute(&mut self.db)
        .await?;

        Ok(())
    }

    pub async fn edit_note(
        &mut self,
        id: i64,
        fach_id: i64,
        schriftlich: Option<i64>,
        muendlich: Option<i64>,
        gewichtung: f64,
    ) -> Result<()> {
        let insgesamt = match (schriftlich, muendlich) {
            (Some(s), Some(m)) => Some((s as f64) * (1.0 - gewichtung) + (m as f64) * gewichtung),
            _ => None,
        };

        query!("UPDATE Note SET schriftlich = ?1, muendlich = ?2, gewichtung = ?3, insgesamt = ?4 WHERE id = ?5",
            schriftlich,
            muendlich,
            gewichtung,
            insgesamt,
            id
        )
        .execute(&mut self.db)
        .await?;

        query!(
            "UPDATE ZeitraumHatNote SET fach_id = ?1 WHERE note_id = ?2",
            fach_id,
            id
        )
        .execute(&mut self.db)
        .await?;

        Ok(())
    }

    pub async fn delete_zeitraum(&mut self, zeitraum_id: i64) -> Result<()> {
        // Get all relations
        let note_ids = query_as!(
            NoteId,
            "SELECT note_id FROM ZeitraumHatNote WHERE zeitraum_id = ?1",
            zeitraum_id,
        )
        .fetch_all(&mut self.db)
        .await?;

        // Delete from the relations-table
        query!(
            "DELETE FROM ZeitraumHatNote WHERE zeitraum_id = ?1",
            zeitraum_id
        )
        .execute(&mut self.db)
        .await?;

        // Delete all notes with relations
        if !note_ids.is_empty() {
            for NoteId { note_id } in &note_ids {
                query!("DELETE FROM Note WHERE id = ?1", note_id)
                    .execute(&mut self.db)
                    .await?;
            }
        }

        // Delete Zeitraum
        query!("DELETE FROM Zeitraum WHERE id = ?1", zeitraum_id)
            .execute(&mut self.db)
            .await?;

        Ok(())
    }

    pub async fn delete_fach(&mut self, fach_id: i64) -> Result<()> {
        // Get all relations
        let note_ids = query_as!(
            NoteId,
            "SELECT note_id FROM ZeitraumHatNote WHERE fach_id = ?1",
            fach_id,
        )
        .fetch_all(&mut self.db)
        .await?;

        // Delete from the relations-table
        query!("DELETE FROM ZeitraumHatNote WHERE fach_id = ?1", fach_id)
            .execute(&mut self.db)
            .await?;

        // Delete all notes with relations
        if !note_ids.is_empty() {
            for NoteId { note_id } in &note_ids {
                query!("DELETE FROM Note WHERE id = ?1", note_id)
                    .execute(&mut self.db)
                    .await?;
            }
        }

        // Delete Fach
        query!("DELETE FROM Fach WHERE id = ?1", fach_id)
            .execute(&mut self.db)
            .await?;

        Ok(())
    }

    pub async fn delete_note(&mut self, note_id: i64) -> Result<()> {
        // Delete from relations table
        query!("DELETE FROM ZeitraumHatNote WHERE note_id = ?1", note_id)
            .execute(&mut self.db)
            .await?;

        // Delete Note
        query!("DELETE FROM Note WHERE id = ?1", note_id)
            .execute(&mut self.db)
            .await?;
        Ok(())
    }
}

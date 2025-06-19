import eel, sqlite3

# datenbank connecten
conn = sqlite3.connect("noten.db")
conn.row_factory = sqlite3.Row
cursor = conn.cursor()

# tabellen
cursor.execute("""CREATE TABLE IF NOT EXISTS "Fach" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"lehrer"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
)""")

cursor.execute("""CREATE TABLE IF NOT EXISTS "Note" (
	"id"	INTEGER NOT NULL,
	"schriftlich"	INTEGER,
	"muendlich"	INTEGER,
	"gewichtung"	INTEGER NOT NULL,
	"insgesamt"	INTEGER,
	PRIMARY KEY("id" AUTOINCREMENT)
)""")

cursor.execute("""CREATE TABLE IF NOT EXISTS "Zeitraum" (
	"id"	INTEGER NOT NULL,
	"quartal" INTEGER NOT NULL,
	"stufe" INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
)""")

cursor.execute("""CREATE TABLE IF NOT EXISTS "ZeitraumHatNote" (
	"zeitraum_id"	INTEGER NOT NULL,
	"fach_id"	INTEGER NOT NULL,
	"note_id"	INTEGER NOT NULL,
	PRIMARY KEY("zeitraum_id","fach_id","note_id")
)""")

eel.init('web')

def dictionarize(cursor: sqlite3.Cursor):
    return [dict(row) for row in cursor.fetchall()]

@eel.expose
def get_zeitraum():
	cursor.execute("SELECT * FROM Zeitraum")
	return dictionarize(cursor)

@eel.expose
def get_fächer():
	cursor.execute("SELECT * FROM Fach")
	return dictionarize(cursor)

@eel.expose
def get_fachnoten_by_zeitraum(id: int):
	cursor.execute("""SELECT *
FROM ZeitraumHatNote
JOIN Note ON Note.id = ZeitraumHatNote.note_id
JOIN Fach ON Fach.id = ZeitraumHatNote.fach_id
WHERE ZeitraumHatNote.zeitraum_id = (?)
""", (id,))
	return dictionarize(cursor)

@eel.expose
def add_zeitraum(quartal: int, stufe: int):
    cursor.execute("INSERT INTO Zeitraum (quartal, stufe) VALUES (?, ?)", (quartal, stufe))
    conn.commit()

@eel.expose
def add_fach(name: str, lehrer: str):
	cursor.execute("INSERT INTO Fach (name, lehrer) VALUES (?, ?)", (name, lehrer))
	conn.commit()

@eel.expose
def add_note(zeitraum_id: int, fach_id: int, schriftlich: int | None, muendlich: int | None, gewichtung: float):
	insgesamt = schriftlich * (1 - gewichtung) + muendlich * gewichtung if schriftlich and muendlich else None
	cursor.execute("INSERT INTO Note (schriftlich, muendlich, gewichtung, insgesamt) VALUES (?, ?, ?, ?)", (schriftlich, muendlich, gewichtung, insgesamt))
	note_id = cursor.lastrowid
	cursor.execute("INSERT INTO ZeitraumHatNote (zeitraum_id, fach_id, note_id) VALUES (?, ?, ?)", (zeitraum_id, fach_id, note_id))
	conn.commit()

@eel.expose
def edit_zeitraum(id: int, quartal: int, stufe: int):
    cursor.execute("UPDATE Zeitraum SET quartal = ?, stufe = ? WHERE id = ?", (quartal, stufe, id))
    conn.commit()

@eel.expose
def edit_fach(id: int, name: str, lehrer: str):
    cursor.execute("UPDATE Fach SET name = ?, lehrer = ? WHERE id = ?", (name, lehrer, id))
    conn.commit()

@eel.expose
def edit_note(id: int, schriftlich: int, muendlich: int, gewichtung: float):
    insgesamt = schriftlich * (1 - gewichtung) + muendlich * gewichtung
    cursor.execute("UPDATE Note SET schriftlich = ?, muendlich = ?, gewichtung = ?, insgesamt = ? WHERE id = ?", (schriftlich, muendlich, gewichtung, insgesamt, id))
    conn.commit()

@eel.expose
def delete_zeitraum(zeitraum_id: int):
    # Alle Relationen kriegen
    cursor.execute("SELECT note_id FROM ZeitraumHatNote WHERE zeitraum_id = ?", (zeitraum_id,))
    note_ids = dictionarize(cursor)

    # Aus Relationstabelle löschen
    cursor.execute("DELETE FROM ZeitraumHatNote WHERE zeitraum_id = ?", (zeitraum_id,))

    # Alle noten mit relation löschen
    if note_ids:
        cursor.executemany("DELETE FROM Note WHERE id = ?", [(note_id["note_id"],) for note_id in note_ids])

    # Zeitraum löschen
    cursor.execute("DELETE FROM Zeitraum WHERE id = ?", (zeitraum_id,))
    conn.commit()

@eel.expose
def delete_fach(fach_id: int):
    # Alle Relationen kriegen
    cursor.execute("SELECT note_id FROM ZeitraumHatNote WHERE fach_id = ?", (fach_id,))
    note_ids = dictionarize(cursor)

    # Aus Relationstabelle löschen
    cursor.execute("DELETE FROM ZeitraumHatNote WHERE fach_id = ?", (fach_id,))

    # Alle noten mit relation löschen
    if note_ids:
        cursor.executemany("DELETE FROM Note WHERE id = ?", [(note_id["note_id"],) for note_id in note_ids])

    # Fach löschen
    cursor.execute("DELETE FROM Fach WHERE id = ?", (fach_id,))
    conn.commit()


@eel.expose
def delete_note(note_id: int):
    cursor.execute("DELETE FROM ZeitraumHatNote WHERE note_id = ?", (note_id,))
    cursor.execute("DELETE FROM Note WHERE id = ?", (note_id,))
    conn.commit()

eel.start('main.html')

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
	"schriftlich"	INTEGER NOT NULL,
	"muendlich"	INTEGER NOT NULL,
	"gewichtung"	INTEGER NOT NULL,
	"insgesamt"	INTEGER NOT NULL,
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
def add_zeitraum(quartal, stufe): 
	cursor.execute("INSERT INTO Zeitraum (quartal, stufe) VALUES (?, ?)", (quartal, stufe))
	conn.commit()

@eel.expose
def add_fach(name, lehrer):
	cursor.execute("INSERT INTO Fach (name, lehrer) VALUES (?, ?)", (name, lehrer))
	conn.commit()

@eel.expose
def add_note(zeitraum_id, fach_id, schriftlich, muendlich, gewichtung) -> None:
	insgesamt = schriftlich * (1 - gewichtung) + muendlich * gewichtung
	cursor.execute("INSERT INTO Note (schriftlich, muendlich, gewichtung, insgesamt) VALUES (?, ?, ?, ?)", (schriftlich, muendlich, gewichtung, insgesamt))
	note_id = cursor.lastrowid
	cursor.execute("INSERT INTO ZeitraumHatNote (zeitraum_id, fach_id, note_id) VALUES (?, ?, ?)", (zeitraum_id, fach_id, note_id))
	conn.commit()

eel.start('main.html')

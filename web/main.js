init();

async function init() {
	console.log("Running init...");
	console.log(await eel.get_zeitraum()());
}

async function add_note() {
	const subject = document.getElementById("subjectInput");
	const grade = document.getElementById("gradeInput");

	//await add_note(zeitraum_id, fach_id, schriftlich, muendlich, gewichtung)()
	await eel.add_note(1, 1, 13, 15, 0.6)();
}

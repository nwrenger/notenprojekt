// State management
const state = {
	zeiträume: [],
	fächer: [],
	noten: [],
	selectedZeitraum: null,
};
const listeners = {};

/**
 * Use to subscribe changes of a prop to the html.
 * @param {string} prop - The prop name.
 * @param {(value) => void} fn - The function called when the value gets updated. The so called update function.
 */
function subscribe(prop, fn) {
	listeners[prop] = listeners[prop] || [];
	listeners[prop].push(fn);
}

/**
 * Use when updating any state, calls the update function set by the subscribe function.
 * @param {any} updates - The fields of states which got updated and should be newly set.
 */
function setState(updates) {
	Object.assign(state, updates);
	Object.keys(updates).forEach((prop) => {
		(listeners[prop] || []).forEach((fn) => fn(state[prop]));
	});
}

// Use to load initially and reload any data
async function loadData() {
	console.log("Running init...");
	console.log(await eel.get_zeitraum()());
	let fächer = await eel.get_fächer()();
	setState({ fächer: fächer });
}
loadData();

subscribe("zeiträume", (values) => {});

subscribe("fächer", (values) => {
	let el = document.getElementById("fach-input");
	if (el) {
		// Delete all options, not the "Fach" option
		let children = Array.from(el.children);
		for (const child of children) {
			console.log(child);
			if (child.value != "selector") {
				el.removeChild(child);
			}
		}

		// Add the updated fächer
		for (const { id, name, lehrer } of values) {
			let option = document.createElement("option");
			option.value = id;
			option.textContent = `${name} (${lehrer})`;
			el.appendChild(option);
		}
	}
});

async function addNote() {
	const fach = document.getElementById("fach-input");
	const mündlich = document.getElementById("mündlich-note-input");
	const schriftlich = document.getElementById("schriftlich-note-input");
	const gewichtung = document.getElementById("gewichtung-input");

	if (fach && mündlich && schriftlich && gewichtung) {
		let fachValue = parseInt(fach.value);
		let mündlichValue = parseInt(mündlich.value);
		let schriftlichValue = parseInt(schriftlich.value);
		let gewichtungValue = parseFloat(gewichtung.value);

		if (fachValue && mündlichValue && schriftlichValue && gewichtungValue) {
			await eel.add_note(
				1,
				fachValue,
				schriftlichValue,
				mündlichValue,
				gewichtungValue,
			)();

			// reset
			fach.selectedIndex = 0;
			mündlich.selectedIndex = 0;
			schriftlich.selectedIndex = 0;
		} else {
			alert("Alle Felder müssen ausgewählte felder haben!");
		}
	}
}

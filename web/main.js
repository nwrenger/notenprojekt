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

subscribe("fächer", (values) => {
	let el = document.getElementById("subjectInput");
	if (el) {
		// Delete all options, not the "Fach" option
		let children = Array.from(el.children);
		for (const child of children) {
			console.log(child);
			if (child.value == child.textContent) {
				el.removeChild(child);
			}
		}

		// Add the updated fächer
		for (const { id, name } of values) {
			let option = document.createElement("option");
			option.value = id;
			option.textContent = name;
			el.appendChild(option);
		}
	}
});

async function addNote() {
	const subject = document.getElementById("subjectInput");
	const grade = document.getElementById("gradeInput");

	//await add_note(zeitraum_id, fach_id, schriftlich, muendlich, gewichtung)()
	await eel.add_note(1, 1, 13, 15, 0.6)();
}

// State management
const state = {
	zeiträume: [],
	fächer: [],
	noten: [],
	selectedTab: null,
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
	let zeiträume = await eel.get_zeitraum()();
	let fächer = await eel.get_fächer()();
	setState({ selectedTab: 1, zeiträume: zeiträume, fächer: fächer });
}
loadData();

subscribe("selectedTab", (id) => {
	if (id) {
		let node_id = typeof id == "number" ? `period-${id}` : "tab-edit";
		let el = document.getElementById("tabs");
		let newly_selected = document.getElementById(node_id);
		if (el && newly_selected) {
			for (const child of el.children) {
				child.classList.remove("active");
			}
			newly_selected.classList.add("active");
		}
		let grades_view = document.getElementById("grades-view");
		let edit_view = document.getElementById("edit-view");
		if (grades_view && edit_view) {
			if (typeof id == "number") {
				grades_view.classList.remove("hidden");
				edit_view.classList.add("hidden");
				// TODO
			} else if (id == "edit") {
				grades_view.classList.add("hidden");
				edit_view.classList.remove("hidden");
				// TODO
			}
		}
	}
});

subscribe("zeiträume", (values) => {
	let el = document.getElementById("tabs");
	if (el) {
		// Delete all tabs, not the "Bearbeiten" tab
		let children = Array.from(el.children);
		for (const child of children) {
			console.log(child);
			if (child.id != "tab-edit") {
				el.removeChild(child);
			}
		}

		// Add the updated zeiträume
		for ({ id, quartal, stufe } of values) {
			let node = document.createElement("div");
			node.classList.add("tab");
			node.addEventListener(
				"click",
				((currentId) => {
					return () => {
						setState({ selectedTab: currentId });
					};
				})(id),
			);

			node.id = `period-${id}`;
			node.textContent = `Stufe ${stufe}.${quartal}`;
			el.prepend(node);
		}
	}
});

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

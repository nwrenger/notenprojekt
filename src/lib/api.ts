import { invoke } from "@tauri-apps/api/core";

namespace api {
	export type Error =
		| { kind: "PoisonedLock" }
		| { kind: "Tauri"; value: string }
		| { kind: "Database"; value: string }
		| { kind: "FileSystem"; value: string };

	export interface Zeitraum {
		id: number;
		quartal: number;
		stufe: number;
	}

	export interface Fach {
		id: number;
		name: string;
		lehrer: string | undefined;
	}

	export interface Note {
		id: number;
		fach_id: number;
		muendlich: number | undefined;
		schriftlich: number | undefined;
		gewichtung: number;
		insgesamt: number | undefined;
	}

	export async function get_zeitraeume(): Promise<Zeitraum[]> {
		return await invoke("get_zeitraeume");
	}

	export async function get_faecher(): Promise<Fach[]> {
		return await invoke("get_faecher");
	}

	// Nils stinkt
	export async function get_noten(zeitraum_id: number): Promise<Note[]> {
		return await invoke("get_noten", {
			zeitraumId: zeitraum_id,
		});
	}

	export async function add_zeitraum(
		quartal: number,
		stufe: number,
	): Promise<void> {
		return await invoke("add_zeitraum", { quartal: quartal, stufe: stufe });
	}

	export async function add_fach(
		name: string,
		lehrer: string | undefined,
	): Promise<void> {
		return await invoke("add_fach", { name: name, lehrer: lehrer });
	}

	export async function add_note(
		zeitraum_id: number,
		fach_id: number,
		muendlich: number | undefined,
		schriftlich: number | undefined,
		gewichtung: number,
	): Promise<void> {
		return await invoke("add_note", {
			zeitraumId: zeitraum_id,
			fachId: fach_id,
			muendlich: muendlich,
			schriftlich: schriftlich,
			gewichtung: gewichtung,
		});
	}

	export async function edit_zeitraum(
		id: number,
		quartal: number,
		stufe: number,
	): Promise<void> {
		return await invoke("edit_zeitraum", {
			id: id,
			quartal: quartal,
			stufe: stufe,
		});
	}

	export async function edit_fach(
		id: number,
		name: string,
		lehrer: string | undefined,
	): Promise<void> {
		return await invoke("edit_fach", {
			id: id,
			name: name,
			lehrer: lehrer,
		});
	}

	export async function edit_note(
		id: number,
		fach_id: number,
		muendlich: number | undefined,
		schriftlich: number | undefined,
		gewichtung: number,
	): Promise<void> {
		return await invoke("edit_note", {
			id: id,
			fachId: fach_id,
			schriftlich: schriftlich,
			muendlich: muendlich,
			gewichtung: gewichtung,
		});
	}

	export async function delete_zeitraum(zeitraum_id: number): Promise<void> {
		return await invoke("delete_zeitraum", {
			zeitraumId: zeitraum_id,
		});
	}

	export async function delete_fach(fach_id: number): Promise<void> {
		return await invoke("delete_fach", {
			fachId: fach_id,
		});
	}

	export async function delete_note(note_id: number): Promise<void> {
		return await invoke("delete_note", {
			noteId: note_id,
		});
	}
}

export default api;

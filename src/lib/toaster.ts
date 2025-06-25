import type api from "./api";
import { createToaster } from "@skeletonlabs/skeleton-svelte";
export const toaster = createToaster({ placement: "bottom-end" });

/** Gets `T` of `api.Result<T>` if no error occurred otherwise displays the error via a toast and throws the error */
export async function handle_promise<T>(
	promise: Promise<T>,
	on_error?: () => void,
): Promise<T> | never {
	try {
		return await promise;
	} catch (e: unknown) {
		let error = e as api.Error;

		error_toast(error_msg(error));

		if (on_error) on_error();

		throw error.kind;
	}
}

/** Displays a error toast with improved configuration */
export function error_toast(e: { title: string; description: string }) {
	toaster.error(e);
}

/** Server Error translations */
function error_msg(error: api.Error): { title: string; description: string } {
	switch (error.kind) {
		case "PoisonedLock":
			return {
				title: "Fatal Backend Error",
				description: "An internal lock was poisoned.",
			};
		case "Database":
			return {
				title: "Database Error",
				description: error.value,
			};

		case "FileSystem":
			return { title: "File System Error", description: error.value };
		case "Tauri":
			return { title: "Tauri Runtime Error", description: error.value };
		default:
			return {
				title: "Fatal Error",
				description:
					"An unknown Error has occurred. Try reopening the app!",
			};
	}
}

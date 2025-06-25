import { persisted, type Persisted } from "svelte-persisted-store";

export const selected_tab: Persisted<undefined | string> = persisted(
	"selected_tab",
	undefined,
);

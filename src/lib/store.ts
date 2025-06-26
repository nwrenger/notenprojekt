import { persisted, type Persisted } from "svelte-persisted-store";

export const selected_tab: Persisted<null | string> = persisted(
	"selected_tab",
	null,
);

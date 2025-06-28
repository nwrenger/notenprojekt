<script lang="ts">
	import api from "$lib/api";
	import { handle_promise } from "$lib/toaster";
	import { allDefined } from "$lib/utils";
	import { Modal } from "@skeletonlabs/skeleton-svelte";
	import { Pencil, Plus } from "lucide-svelte";

	interface Props {
		fach?: api.Fach;
		update: () => void;
	}

	let { fach, update }: Props = $props();

	let new_fach = $state({
		name: fach?.name,
		lehrer: fach?.lehrer,
	});
	// $inspect(new_fach);
	let is_new = $derived(!fach);
	let valid = $derived(allDefined(new_fach, ["lehrer"]));
	let openState = $state(false);

	function modalClose() {
		openState = false;

		// reset
		new_fach = {
			name: fach?.name,
			lehrer: fach?.lehrer,
		};
	}

	async function apply() {
		if (allDefined(new_fach, ["lehrer"])) {
			if (is_new) {
				await handle_promise(
					api.add_fach(new_fach.name, new_fach.lehrer),
				);
			} else if (fach) {
				await handle_promise(
					api.edit_fach(fach.id, new_fach.name, new_fach.lehrer),
				);
			}

			update();
			modalClose();
		}
	}
</script>

<Modal
	open={openState}
	onOpenChange={(e) => (openState = e.open)}
	triggerBase="flex items-center"
	contentBase="card bg-surface-100-900 p-4 space-y-4 shadow-xl min-w-md"
	backdropClasses="backdrop-blur-sm"
>
	{#snippet trigger()}
		{#if is_new}
			<button
				class="btn-icon preset-filled-success-500"
				title="Fach hinzufügen?"><Plus /></button
			>
		{:else}
			<button
				class="btn-icon preset-filled-tertiary-500"
				title="Fach bearbeiten?"><Pencil /></button
			>
		{/if}
	{/snippet}
	{#snippet content()}
		<header class="flex justify-between">
			{#if is_new}
				<h4 class="h4">Fach hinzufügen?</h4>
			{:else}
				<h4 class="h4">Fach bearbeiten?</h4>
			{/if}
		</header>
		<article class="space-y-2">
			<label class="label">
				<span class="label-text">Name</span>
				<input
					type="text"
					class="input"
					placeholder="Name eingeben..."
					bind:value={new_fach.name}
				/>
			</label>
			<label class="label">
				<span class="label-text">Lehrer (Optional)</span>
				<input
					type="text"
					class="input"
					placeholder="Lehrer eingeben..."
					bind:value={new_fach.lehrer}
				/>
			</label>
		</article>
		<footer class="flex justify-end gap-4">
			<button type="button" class="btn preset-tonal" onclick={modalClose}
				>Abbrechen</button
			>
			<button
				type="button"
				class="btn preset-filled"
				onclick={apply}
				disabled={!valid}
			>
				{#if is_new}
					Hinzufügen
				{:else}
					Übernehmen
				{/if}
			</button>
		</footer>
	{/snippet}
</Modal>

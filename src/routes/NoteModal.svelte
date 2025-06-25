<script lang="ts">
	import api from "$lib/api";
	import { handle_promise } from "$lib/toaster";
	import { allDefined } from "$lib/utils";
	import { Modal } from "@skeletonlabs/skeleton-svelte";
	import { Pencil, Plus } from "lucide-svelte";

	interface Props {
		zeitraum_id: number;
		fach_id?: number;
		note: api.Note | undefined;
		faecher: api.Fach[];
		update: () => void;
	}

	let { zeitraum_id, fach_id, note, faecher, update }: Props = $props();

	let new_note = $state({
		fach_id: fach_id,
		schriftlich: note?.schriftlich,
		muendlich: note?.muendlich,
		gewichtung: note?.gewichtung,
	});
	$inspect(new_note);
	let is_new = $derived(!note);
	let valid = $derived(allDefined(new_note, ["schriftlich", "muendlich"]));
	let openState = $state(false);

	function modalClose() {
		openState = false;

		// reset
		new_note = {
			fach_id: undefined,
			schriftlich: note?.schriftlich,
			muendlich: note?.muendlich,
			gewichtung: note?.gewichtung,
		};
	}

	async function apply() {
		if (new_note.fach_id && new_note.gewichtung) {
			if (is_new) {
				await handle_promise(
					api.add_note(
						zeitraum_id,
						new_note.fach_id,
						new_note.schriftlich,
						new_note.muendlich,
						new_note.gewichtung,
					),
				);
			} else if (note) {
				await handle_promise(
					api.edit_note(
						note.id,
						new_note.fach_id,
						new_note.schriftlich,
						new_note.muendlich,
						new_note.gewichtung,
					),
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
				title="Note hinzufügen?"><Plus /></button
			>
		{:else}
			<button
				class="btn-icon preset-filled-tertiary-500"
				title="Note bearbeiten?"><Pencil /></button
			>
		{/if}
	{/snippet}
	{#snippet content()}
		<header class="flex justify-between">
			{#if is_new}
				<h4 class="h4">Note hinzufügen?</h4>
			{:else}
				<h4 class="h4">Note bearbeiten?</h4>
			{/if}
		</header>
		<article class="space-y-2">
			<label class="label">
				<span class="label-text">Fach</span>
				<select class="select" bind:value={new_note.fach_id}>
					{#each faecher as fach}
						<option value={fach.id}
							>{fach.name} - {fach.lehrer}</option
						>
					{/each}
				</select>
			</label>
			<div class="grid grid-cols-2 gap-1.5">
				<label class="label">
					<span class="label-text">Mündliche Note</span>
					<select class="select" bind:value={new_note.muendlich}>
						<option value={null}>Leer</option>
						{#each Array.from({ length: 15 }) as _, i}
							{@const num = 15 - i}
							<option value={num}>{num}</option>
						{/each}
					</select>
				</label>
				<label class="label">
					<span class="label-text">Schriftliche Note</span>
					<select class="select" bind:value={new_note.schriftlich}>
						<option value={null}>Leer</option>
						{#each Array.from({ length: 15 }) as _, i}
							{@const num = 15 - i}
							<option value={num}>{num}</option>
						{/each}
					</select>
				</label>
			</div>
			<label class="label">
				<span class="label-text">Gewichtung</span>
				<select class="select" bind:value={new_note.gewichtung}>
					<option value={0.5}>50/50</option>
					<option value={0.6}>60/40</option>
					<option value={0.7}>70/30</option>
				</select>
			</label>
		</article>
		<footer class="flex justify-end gap-4">
			<button type="button" class="btn preset-tonal" onclick={modalClose}
				>Cancel</button
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

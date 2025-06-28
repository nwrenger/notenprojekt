<script lang="ts">
	import api from "$lib/api";
	import { handle_promise } from "$lib/toaster";
	import { allDefined } from "$lib/utils";
	import { Modal } from "@skeletonlabs/skeleton-svelte";
	import { Pencil, Plus } from "lucide-svelte";

	interface Props {
		zeitraum_id: number;
		note?: api.Note;
		future_noten?: Promise<api.Note[]>;
		faecher: api.Fach[];
		update: () => void;
	}

	let { zeitraum_id, future_noten, note, faecher, update }: Props = $props();

	let new_note = $state({
		fach_id: note?.fach_id,
		schriftlich: note?.schriftlich,
		muendlich: note?.muendlich,
		gewichtung: note?.gewichtung,
	});
	// $inspect(new_note);
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
		if (allDefined(new_note, ["schriftlich", "muendlich"])) {
			if (is_new) {
				await handle_promise(
					api.add_note(
						zeitraum_id,
						new_note.fach_id,
						new_note.muendlich,
						new_note.schriftlich,
						new_note.gewichtung,
					),
				);
			} else if (note) {
				await handle_promise(
					api.edit_note(
						note.id,
						new_note.fach_id,
						new_note.muendlich,
						new_note.schriftlich,
						new_note.gewichtung,
					),
				);
			}

			update();
			modalClose();
		}
	}

	async function initNewNoteFach() {
		if (!future_noten) return;

		const existing: api.Note[] = await handle_promise(future_noten);
		const usedIds = new Set(existing.map((n) => n.fach_id));
		const next = faecher.find((f) => !usedIds.has(f.id));
		new_note.fach_id = next?.id ?? faecher[0]?.id;
	}

	$effect(() => {
		if (is_new && openState) initNewNoteFach();
	});
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
						<option value={fach.id}>
							{fach.name}
							{#if fach.lehrer}
								- {fach.lehrer}
							{/if}
						</option>
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
					<option selected value={0.6}>60/40</option>
					<option value={0.7}>70/30</option>
				</select>
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

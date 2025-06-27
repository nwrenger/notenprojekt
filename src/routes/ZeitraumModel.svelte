<script lang="ts">
	import api from "$lib/api";
	import { handle_promise } from "$lib/toaster";
	import { allDefined } from "$lib/utils";
	import { Modal } from "@skeletonlabs/skeleton-svelte";
	import { Pencil, Plus } from "lucide-svelte";

	interface Props {
		zeitraum?: api.Zeitraum;
		update: () => void;
	}

	let { zeitraum, update }: Props = $props();

	let new_zeitraum = $state({
		stufe: zeitraum?.stufe,
		quartal: zeitraum?.quartal,
	});
	// $inspect(new_zeitraum);
	let is_new = $derived(!zeitraum);
	let valid = $derived(allDefined(new_zeitraum, []));
	let openState = $state(false);

	function modalClose() {
		openState = false;

		// reset
		new_zeitraum = {
			stufe: zeitraum?.stufe,
			quartal: zeitraum?.quartal,
		};
	}

	async function apply() {
		if (allDefined(new_zeitraum, [])) {
			if (is_new) {
				await handle_promise(
					api.add_zeitraum(new_zeitraum.quartal, new_zeitraum.stufe),
				);
			} else if (zeitraum) {
				await handle_promise(
					api.edit_zeitraum(
						zeitraum.id,
						new_zeitraum.quartal,
						new_zeitraum.stufe,
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
				title="Zeitraum hinzufügen?"><Plus /></button
			>
		{:else}
			<button
				class="btn-icon preset-filled-tertiary-500"
				title="Zeitraum bearbeiten?"><Pencil /></button
			>
		{/if}
	{/snippet}
	{#snippet content()}
		<header class="flex justify-between">
			{#if is_new}
				<h4 class="h4">Zeitraum hinzufügen?</h4>
			{:else}
				<h4 class="h4">Zeitraum bearbeiten?</h4>
			{/if}
		</header>
		<article class="space-y-2">
			<label class="label">
				<span class="label-text">Quartal</span>
				<input
					type="number"
					class="input"
					placeholder="Quartal eingeben..."
					bind:value={new_zeitraum.quartal}
				/>
			</label>
			<label class="label">
				<span class="label-text">Stufe</span>
				<input
					type="number"
					class="input"
					placeholder="Stufe eingeben..."
					bind:value={new_zeitraum.stufe}
				/>
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

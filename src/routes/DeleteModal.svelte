<script lang="ts">
	import { handle_promise } from "$lib/toaster";
	import { Modal } from "@skeletonlabs/skeleton-svelte";
	import { Trash2 } from "lucide-svelte";

	interface Props {
		message: string;
		del: () => Promise<void>;
		update: () => void;
	}

	let { message, del, update }: Props = $props();

	let openState = $state(false);

	function modalClose() {
		openState = false;
	}

	async function apply() {
		await handle_promise(del());
		update();
		modalClose();
	}
</script>

<Modal
	open={openState}
	onOpenChange={(e) => (openState = e.open)}
	triggerBase="flex items-center"
	contentBase="card bg-surface-100-900 p-4 space-y-4 shadow-xl min-w-md max-w-md"
	backdropClasses="backdrop-blur-sm"
>
	{#snippet trigger()}
		<button class="btn-icon preset-filled-error-500" title="Löschen?"
			><Trash2 /></button
		>
	{/snippet}
	{#snippet content()}
		<header class="flex justify-between">
			<h4 class="h4">Löschen?</h4>
		</header>
		<article class="space-y-2">
			<p class="opacity-80">{message}</p>
		</article>
		<footer class="flex justify-end gap-4">
			<button type="button" class="btn preset-tonal" onclick={modalClose}
				>Cancel</button
			>
			<button
				type="button"
				class="btn preset-filled-error-500"
				onclick={apply}
			>
				Löschen
			</button>
		</footer>
	{/snippet}
</Modal>

<script lang="ts">
	import { Toaster } from "@skeletonlabs/skeleton-svelte";
	import "../app.css";
	import { toaster } from "$lib/toaster";

	let { children } = $props();
</script>

<Toaster {toaster}></Toaster>

{@render children()}

<script lang="ts">
	import api from "$lib/api";
	import { Tabs } from "@skeletonlabs/skeleton-svelte";
	import {
		ChartNoAxesCombined,
		FileQuestionMark,
		Settings2,
		TriangleAlert,
	} from "lucide-svelte";
	import NoteModal from "./NoteModal.svelte";
	import DeleteModal from "./DeleteModal.svelte";
	import { handle_promise } from "$lib/toaster";
	import { selected_tab } from "$lib/store";

	let future_zeitraeume: undefined | Promise<api.Zeitraum[]> = $state();
	let future_fachnoten: undefined | Promise<[api.Fach, api.Note][]> =
		$state();
	// make sure changes of $selected_tab reloads future_fachnoten
	$effect(loadFachnoten);
	let future_faecher: undefined | Promise<api.Fach[]> = $state();

	// Once initially
	loadAll();
	function loadAll() {
		loadZeitraeume();
		loadFachnoten();
		loadFaecher();
	}
	function loadZeitraeume() {
		future_zeitraeume = api.get_zeitraeume();
	}
	function loadFachnoten() {
		if (
			$selected_tab &&
			$selected_tab != "edit" &&
			$selected_tab != "analysis"
		)
			future_fachnoten = api.get_fachnoten_by_zeitraum(
				parseInt($selected_tab),
			);
	}
	function loadFaecher() {
		future_faecher = api.get_faecher();
	}
</script>

<div class="p-2 h-full">
	<Tabs
		value={$selected_tab}
		onValueChange={(e) => ($selected_tab = e.value)}
		fluid
		composite
		listClasses="preset-tonal pt-2 px-2 rounded-md whitespace-nowrap overflow-x-auto overflow-y-clip"
		contentClasses="overflow-y-scroll h-[calc(100%-61px)]"
		classes="h-full"
	>
		{#snippet list()}
			{#if future_zeitraeume}
				{#await handle_promise(future_zeitraeume) then zeitraeume}
					{#each zeitraeume as zeitraum}
						<Tabs.Control
							value={zeitraum.id.toString()}
							labelBase="btn hover:filter-none!"
						>
							<div class="w-10 sm:w-full">
								<div class="truncate">
									{zeitraum.stufe}.{zeitraum.quartal}
								</div>
							</div>
						</Tabs.Control>
					{/each}
				{/await}
			{/if}
			<Tabs.Control value="analysis" labelBase="btn hover:filter-none!">
				{#snippet lead()}<ChartNoAxesCombined size="18" />{/snippet}
				<div class="w-8 sm:w-full">
					<div class="truncate">Analyse</div>
				</div>
			</Tabs.Control>
			<Tabs.Control value="edit" labelBase="btn hover:filter-none!">
				{#snippet lead()}<Settings2 size="18" />{/snippet}
				<div class="w-8 sm:w-full">
					<div class="truncate">Bearbeiten</div>
				</div>
			</Tabs.Control>
		{/snippet}
		{#snippet content()}
			<!-- If not tab got selected, display a small welcoming message -->
			{#if !$selected_tab}
				<p class="px-2 opacity-80 italic">
					Willkommen im Notenmanager von Hanno Dehmel und Nils
					Wrenger! Wähle einen Tab aus, um deine Noten anzuzeigen oder
					zu bearbeiten. Wenn du den Notenmanager zum ersten Mal
					startest, solltest du den Bearbeiten Tab auswählen und
					erstmal alle Fächer und die jeweiligen Zeiträume erstellen!
				</p>
			{/if}
			{#if future_zeitraeume && future_faecher}
				{#await handle_promise(Promise.all( [future_zeitraeume, future_faecher], )) then [zeitraeume, faecher]}
					{#each zeitraeume as zeitraum}
						<Tabs.Panel value={zeitraum.id.toString()}>
							<div
								class="fixed bottom-0 left-0 z-50 p-4 backdrop-blur-xs rounded-tr-md"
							>
								<NoteModal
									zeitraum_id={zeitraum.id}
									note={undefined}
									{faecher}
									update={loadFachnoten}
								/>
							</div>
							{#if future_fachnoten}
								{#await handle_promise(future_fachnoten) then fachnotenUnsortiert}
									{@const fachnoten =
										fachnotenUnsortiert.toSorted(
											([_, a], [_2, b]) =>
												(b?.insgesamt || 0) -
												(a?.insgesamt || 0),
										)}
									<ul class="list-inside list-none px-2">
										{#each fachnoten as fachnote}
											<li
												class="flex justify-between py-3 border-b-[1px] border-surface-500 last:border-b-0"
											>
												<span
													class="font-bold flex items-center"
												>
													{fachnote[0].name}
													<span
														class="ps-2 opacity-60 text-xs"
														>{fachnote[0]
															.lehrer}</span
													>
												</span>
												<div
													class="space-x-2 flex justify-center items-center"
												>
													<span
														class="badge preset-filled-primary-500 h6 h-[39px]"
													>
														{#if fachnote[1].insgesamt}
															{fachnote[1].insgesamt.toFixed(
																1,
															)}
														{:else}
															<FileQuestionMark
																size={32}
																class="py-1"
															/>
														{/if}
													</span>
													<div
														class="flex justify-center items-center space-x-1"
													>
														<NoteModal
															zeitraum_id={zeitraum.id}
															fach_id={fachnote[0]
																.id}
															note={fachnote[1]}
															{faecher}
															update={loadFachnoten}
														/>
														<DeleteModal
															message="Wollen Sie die Note unwiederruflich löschen?"
															del={() =>
																api.delete_note(
																	fachnote[1]
																		.id,
																)}
															update={loadFachnoten}
														/>
													</div>
												</div>
											</li>
										{:else}
											<p class="opacity-80 italic">
												Es wurden keine Noten in dem
												ausgewähltem Zeitraum gefunden.
												Noten können über den grünen
												Plus Button hinzugefügt werden!
											</p>
										{/each}
									</ul>
								{/await}
							{/if}
						</Tabs.Panel>
					{/each}
				{/await}
			{/if}
			<Tabs.Panel value="analysis">
				<div
					class="card preset-outlined-warning-500 grid items-center gap-4 p-4 grid-cols-[auto_1fr_auto] mx-2"
				>
					<TriangleAlert />
					<div>
						<h6 class="h6 font-bold">Achtung</h6>
						<p class="opacity-80">
							Dieser Bereich ist noch in der Entwicklung und noch
							nicht fertig!
						</p>
					</div>
				</div>
			</Tabs.Panel>
			<Tabs.Panel value="edit">
				<div
					class="card preset-outlined-warning-500 grid items-center gap-4 p-4 grid-cols-[auto_1fr_auto] mx-2"
				>
					<TriangleAlert />
					<div>
						<h6 class="h6 font-bold">Achtung</h6>
						<p class="opacity-80">
							Dieser Bereich ist noch in der Entwicklung und noch
							nicht fertig!
						</p>
					</div>
				</div>
			</Tabs.Panel>
		{/snippet}
	</Tabs>
</div>

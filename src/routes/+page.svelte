<script lang="ts">
	import api from "$lib/api";
	import { Tabs } from "@skeletonlabs/skeleton-svelte";
	import {
		ChartNoAxesCombined,
		FileQuestionMark,
		Pencil,
		Plus,
		Settings2,
		TriangleAlert,
	} from "lucide-svelte";
	import NoteModal from "./NoteModal.svelte";
	import DeleteModal from "./DeleteModal.svelte";
	import { handle_promise } from "$lib/toaster";
	import { selected_tab } from "$lib/store";
	import ZeitraumModel from "./ZeitraumModel.svelte";
	import FachModel from "./FachModel.svelte";

	let future_zeitraeume: undefined | Promise<api.Zeitraum[]> = $state();
	let future_noten: undefined | Promise<api.Note[]> = $state();
	// make sure changes of $selected_tab reloads future_fachnoten
	$effect(loadNoten);
	let future_faecher: undefined | Promise<api.Fach[]> = $state();

	// Once initially
	loadAll();
	function loadAll() {
		loadZeitraeume();
		loadNoten();
		loadFaecher();
	}
	function loadZeitraeume() {
		future_zeitraeume = api.get_zeitraeume();
	}
	function loadNoten() {
		if (
			$selected_tab &&
			$selected_tab != "edit" &&
			$selected_tab != "analysis"
		)
			future_noten = api.get_noten(parseInt($selected_tab));
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
		listClasses="preset-tonal pt-2 px-2 rounded-md whitespace-nowrap"
		contentClasses="h-[calc(100%-61px)]"
		classes="h-full"
	>
		{#snippet list()}
			{#if future_zeitraeume}
				{#await handle_promise(future_zeitraeume) then zeitraeumeUnsortiert}
					{@const zeitraeume = zeitraeumeUnsortiert.toSorted(
						(a, b) => {
							if (a.stufe == b.stufe) {
								return a.quartal - b.quartal;
							} else {
								return a.stufe - b.stufe;
							}
						},
					)}
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
						<Tabs.Panel
							classes="h-full"
							value={zeitraum.id.toString()}
						>
							<div class="fixed bottom-4 left-4 z-40">
								<NoteModal
									zeitraum_id={zeitraum.id}
									{future_noten}
									{faecher}
									update={loadNoten}
								/>
							</div>
							{#if future_noten}
								{#await handle_promise(future_noten) then notenUsortiert}
									{@const noten = notenUsortiert.toSorted(
										(a, b) =>
											(b?.insgesamt || 0) -
											(a?.insgesamt || 0),
									)}
									<ul
										class="list-inside list-none px-2 overflow-y-scroll h-full"
									>
										{#each noten as note}
											{@const fach = faecher.find(
												(f) => f.id == note.fach_id,
											)}
											<li
												class="flex justify-between py-3 border-b-[1px] border-surface-500 last:border-b-0"
											>
												<span
													class="font-bold flex items-center"
													title="{fach?.name}{fach?.lehrer
														? ` - ${fach?.lehrer}`
														: ''}"
												>
													{fach?.name}
													<span
														class="ps-2 opacity-60 text-xs"
														>{fach?.lehrer}</span
													>
												</span>
												<div
													class="space-x-2 flex justify-center items-center"
												>
													<span
														class="badge preset-filled-primary-500 h6 h-[39px]"
													>
														{#if note.insgesamt}
															{note.insgesamt.toFixed(
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
															{note}
															{faecher}
															update={loadNoten}
														/>
														<DeleteModal
															message="Wollen Sie die Note unwiederruflich löschen?"
															del={() =>
																api.delete_note(
																	note.id,
																)}
															update={loadNoten}
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
			<Tabs.Panel classes="h-full" value="edit">
				<div class="px-2 grid grid-cols-2 gap-4 w-full h-full">
					<div class="space-y-2 h-full min-h-0">
						<div class="flex items-center justify-between">
							<h6 class="h6 font-bold">Zeiträume</h6>
							<ZeitraumModel update={loadZeitraeume} />
						</div>

						<div class="h-[calc(100%-41px)] overflow-y-auto">
							<ul
								class="list-inside list-none overflow-y-scroll h-full"
							>
								{#if future_zeitraeume}
									{#await future_zeitraeume then zeitraeume}
										{#each zeitraeume as zeitraum}
											<li
												class="flex justify-between py-3 border-b-[1px] border-surface-500 last:border-b-0"
											>
												<span
													class="font-bold flex items-center"
												>
													{zeitraum.stufe}.{zeitraum.quartal}
												</span>
												<div
													class="flex justify-center items-center space-x-1"
												>
													<ZeitraumModel
														{zeitraum}
														update={loadZeitraeume}
													/>

													<DeleteModal
														message="Wollen Sie den Zeitraum und alle jeweils dazu eingetragenen Noten unwiederruflich löschen?"
														del={() =>
															api.delete_zeitraum(
																zeitraum.id,
															)}
														update={loadZeitraeume}
													/>
												</div>
											</li>
										{/each}
									{/await}
								{/if}
							</ul>
						</div>
					</div>

					<div class="space-y-2 h-full min-h-0">
						<div class="space-y-2 h-full min-h-0">
							<div class="flex items-center justify-between">
								<h6 class="h6 font-bold">Fächer</h6>
								<FachModel update={loadFaecher} />
							</div>

							<div class="h-[calc(100%-41px)] overflow-y-auto">
								<ul
									class="list-inside list-none overflow-y-scroll h-full"
								>
									{#if future_faecher}
										{#await future_faecher then faecher}
											{#each faecher as fach}
												<li
													class="flex justify-between py-3 border-b-[1px] border-surface-500 last:border-b-0"
												>
													<span
														class="font-bold flex items-center"
													>
														{fach.name}
														<span
															class="ps-2 opacity-60 text-xs"
															>{fach.lehrer}</span
														>
													</span>
													<div
														class="flex justify-center items-center space-x-1"
													>
														<FachModel
															{fach}
															update={loadFaecher}
														/>

														<DeleteModal
															message="Wollen Sie das Fach und alle jeweils dazu eingetragenen Noten unwiederruflich löschen?"
															del={() =>
																api.delete_fach(
																	fach.id,
																)}
															update={loadFaecher}
														/>
													</div>
												</li>
											{/each}
										{/await}
									{/if}
								</ul>
							</div>
						</div>
					</div>
				</div>
			</Tabs.Panel>
		{/snippet}
	</Tabs>
</div>
